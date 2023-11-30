#![no_std]
#![no_main]

use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    // Start the recursive function with an initial depth
    recursive_stack_allocation(1);

    loop {
        delay.delay_ms(1000u32);
    }
}

fn recursive_stack_allocation(depth: usize) {
    // Allocate some data on the stack
    let stack_data = [0u8; 1024]; // 1 KB of data

    // Use the data to prevent it from being optimized out
    if stack_data[0] == 0 {
        log::info!("Depth: {}, Stack usage: {} bytes", depth, depth * 1024);
    }

    // Recurse to the next depth
    recursive_stack_allocation(depth + 1);

    // Rust compiler might optimize out this recursion unless we add some condition to prevent it.
    if depth > 1000 {
        return;
    }
}
