#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {

    // create variables for gpio pin addresses for d2 led as raw pointers
    const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;

    //  identify bit field to use
    const DIR_OUTPUT_POS: u32 = 0;

    // create configuration value to assign
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    // write configuration values at pointer addresses
    unsafe {
        write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED);
        write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED);
    }

    // create variable for out register
    const GPIO1_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;

    // create variable for value to set in out register
    const GPIO1_OUT_ROW1_POS: u32 = 21;

    // use a bool to track state of led
    let mut is_on: bool = false;

    // write loop to toggle led
    loop {
        unsafe {
            write_volatile(GPIO1_OUT_ADDR, (is_on as u32) << GPIO1_OUT_ROW1_POS);
        }
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
        }
}
