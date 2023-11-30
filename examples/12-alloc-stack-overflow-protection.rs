#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use core::cell::RefCell;

use critical_section::Mutex;
use hal::{
    clock::ClockControl,
    interrupt,
    peripherals::{Peripherals, Interrupt},
    prelude::*,
    Delay,
    assist_debug::DebugAssist
};
use log::{error, info};

use alloc::vec::Vec;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 300 * 1024; // Define heap size
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

// Static variable to hold DebugAssist
static DA: Mutex<RefCell<Option<DebugAssist<'static>>>> = Mutex::new(RefCell::new(None));

fn install_stack_guard(mut da: DebugAssist<'static>, safe_area_size: u32) {
    extern "C" {
        static mut _stack_end: u32;
        static mut _stack_start: u32;
    }
    let stack_low = unsafe { &mut _stack_end as *mut _ as u32 };
    let stack_high = unsafe { &mut _stack_start as *mut _ as u32 };
    info!("Safe stack {} bytes", stack_high - stack_low - safe_area_size);
    da.enable_region0_monitor(stack_low, stack_low + safe_area_size, true, true);

    critical_section::with(|cs| DA.borrow_ref_mut(cs).replace(da));
    interrupt::enable(Interrupt::ASSIST_DEBUG, interrupt::Priority::Priority1).unwrap();
}

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    // get the debug assist driver
    let da = DebugAssist::new(peripherals.ASSIST_DEBUG);

    install_stack_guard(da, 4096); // 4 KB safe area

    recursive_stack_and_heap_allocation(1, 1024); // Initial depth and allocation size

    loop {
        delay.delay_ms(1000u32);
    }
}

fn recursive_stack_and_heap_allocation(depth: usize, allocation_size: usize) {
    // Allocate some data on the stack
    let mut stack_data = [0u8; 1024]; // 1 KB of data on the stack
    stack_data[0] = 1; // Access the array to ensure it's not optimized out

    // Heap allocation
    let mut test_vec:Vec<u8> = Vec::new();
    test_vec.resize(allocation_size, 0);
    test_vec[0] = 1; // Access the array to ensure it's not optimized out

    info!("Depth: {}, Stack usage: {} bytes, Heap allocation: {}, Memory - used: {}; free: {}",
          depth, depth * 1024, allocation_size, ALLOCATOR.used(), ALLOCATOR.free());

    if depth > 1000 {
        return; // Limit recursion depth to prevent stack overflow
    }

    recursive_stack_and_heap_allocation(depth + 1, allocation_size);
    // Keep allocation size the same, since variables are not going out of scope in recursion
}


#[interrupt]
fn ASSIST_DEBUG() {
    critical_section::with(|cs| {
        error!("\n\nPossible Stack Overflow Detected");
        let mut da = DA.borrow_ref_mut(cs);
        let da = da.as_mut().unwrap();
        if da.is_region0_monitor_interrupt_set() {
            let pc = da.get_region_monitor_pc();
            info!("PC = 0x{:x}", pc);
            da.clear_region0_monitor_interrupt();
            da.disable_region0_monitor();
            loop {}
        }
    });
}
