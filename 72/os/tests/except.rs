#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::Init();
    test_main();
    loop {}
}

#[test_case]
fn BreakpointException() {
    osirs::serial_println!("[BreakpointException...]");
    x86_64::instructions::interrupts::int3();
    osirs::serial_println!("[Exception Handled]");
}