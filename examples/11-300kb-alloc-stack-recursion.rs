#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use log::info;

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

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    recursive_stack_and_heap_allocation(1, 1024); // Initial depth and allocation size

    loop {
        delay.delay_ms(1000u32);
    }
}

fn recursive_stack_and_heap_allocation(depth: usize, allocation_size: usize) {
    // Allocate some data on the stack
    let stack_data = [0u8; 1024]; // 1 KB of data on the stack

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
