#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use alloc::vec::Vec;
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

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");

    let mut allocation_size = 1024; // Start with 1 KB

    loop {
        { // Create a new scope to ensure `test_vec` is dropped before we allocate again
            let mut test_vec:Vec<u8> = Vec::new();

            if test_vec.try_reserve(allocation_size).is_ok() {
                test_vec.resize(allocation_size, 0); // Fill the newly allocated space
                log::info!("Allocated {} bytes", allocation_size);
                if !test_vec.is_empty() {
                    test_vec[0] = 1; // Access the array to ensure it's not optimized out
                }
            } else {
                log::error!("Not enough memory to allocate {} more bytes", allocation_size);
                // Handle allocation failure
            }

            log::info!("Used memory: {}; Free memory: {}", ALLOCATOR.used(), ALLOCATOR.free());
        } // `test_vec` goes out of scope here, and its memory is freed

        allocation_size += 1024; // Increase the allocation size by 1 KB for the next iteration
        delay.delay_ms(50u32);
    }
}
