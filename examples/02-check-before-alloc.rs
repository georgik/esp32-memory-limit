#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use alloc::{vec, vec::Vec};
use esp_backtrace as _;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

fn try_allocate(size: usize) -> Option<Vec<u8>> {
    if ALLOCATOR.free() >= size {
        Some(vec![0u8; size])
    } else {
        None
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
    log::info!("Logger is setup");
    log::info!("Used memory: {}; Free memory: {}", ALLOCATOR.used(), ALLOCATOR.free());

    let mut allocation_size = 1024; // Start with 1 KB

    loop {
        if let Some(mut test_vec) = try_allocate(allocation_size) {
            log::info!("Allocated {} bytes", allocation_size);
            log::info!("Used memory: {}; Free memory: {}", ALLOCATOR.used(), ALLOCATOR.free());

            if !test_vec.is_empty() {
                test_vec[0] = 1; // Access the array to ensure it's not optimized out
            }
            allocation_size += 1024; // Increase the allocation size by 1 KB for the next iteration
        } else {
            log::error!("Not enough memory to allocate {} bytes", allocation_size);
        }

        delay.delay_ms(50u32);
    }
}
