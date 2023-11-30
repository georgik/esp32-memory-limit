#![no_std]
#![no_main]

// Based on article: https://esp-rs.github.io/no_std-training/04_1_stack_overflow_protection.html
// Based on example: https://github.com/esp-rs/no_std-training/blob/main/advanced/stack_overflow_detection/examples/stack_overflow_protection.rs

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
use esp_backtrace as _;
use log::{error, info};

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
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    esp_println::logger::init_logger_from_env();

    // get the debug assist driver
    let da = DebugAssist::new(peripherals.ASSIST_DEBUG);

    install_stack_guard(da, 1024); // 1 KB safe area

    recursive_stack_allocation(1);

    loop {
        delay.delay_ms(1000u32);
    }
}

fn recursive_stack_allocation(depth: usize) {
    let stack_data = [0u8; 1024]; // Reduced stack usage per call to allow deeper recursion
    if stack_data[0] == 0 {
        info!("Depth: {}, Stack usage: {} bytes", depth, depth * 1024);
    }
    recursive_stack_allocation(depth + 1);
    // Note: No additional condition for returning, to allow for stack overflow detection
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
