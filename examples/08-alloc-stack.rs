#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use alloc::vec;
use esp_backtrace as _;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use log::info;

use alloc::vec::Vec;

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
    info!("Memory - used: {}; free: {}", ALLOCATOR.used(), ALLOCATOR.free());

    let mut allocation_size = 1024; // Start with 1 KB for heap allocation

    for i in 1.. {
        // Heap allocation
        let mut test_vec = Vec::new();
        test_vec.resize(allocation_size, 0);
        test_vec[0] = 1; // Access the array to ensure it's not optimized out

        // Stack allocation via recursion
        recursive_stack_allocation(i);

        info!("Iteration: {}, Heap allocated: {}, Stack depth: {}, Memory - used: {}; free: {}",
              i, allocation_size, i, ALLOCATOR.used(), ALLOCATOR.free());

        allocation_size += 1024; // Increase heap allocation size by 1 KB each iteration
        delay.delay_ms(50u32);
    }

    loop {
        delay.delay_ms(1000u32);
    }
}

fn recursive_stack_allocation(depth: usize) {
    let stack_data = [0u8; 1024];

    if depth > 10000 {
        return; // Limit recursion depth to prevent stack overflow
    }

    if stack_data[0] == 0 {
        // Use the data to prevent it from being optimized out
        info!("Stack depth: {}, usage: {} bytes", depth, depth * 1024);
    }

    recursive_stack_allocation(depth + 1);
}
