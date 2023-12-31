#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use alloc::vec;
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
use alloc::vec::Vec;
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
        // Method #1: This will result in panic in case of oom
        // let mut a = vec![0u8; allocation_size];

        // Method #2: Check available memory before allocating
        if let Some(mut a) = try_allocate(allocation_size) {
            log::info!("Allocated {} bytes", allocation_size);
            log::info!("Used memory: {}; Free memory: {}", ALLOCATOR.used(), ALLOCATOR.free());

            if !a.is_empty() {
                a[0] = 1; // Access the array to ensure it's not optimized out
            }
            allocation_size += 1024; // Increase the allocation size by 1 KB for the next iteration
        } else {
            log::error!("Not enough memory to allocate {} bytes", allocation_size);
        }

        delay.delay_ms(500u32);
    }
}
