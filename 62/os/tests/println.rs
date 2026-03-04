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
    test_main();
    osirs::QemuQuit(osirs::QemuPass);
}

#[test_case]
fn PrintlnWrapsToNextLine() {
    osirs::println!("{:081x}", 1);
    osirs::println!("{:x}", 2);
}

#[test_case]
fn PrintlnScrollsAndKeepsColor() {
    let LineCount: usize = 30;
    for LineIndex in 0..LineCount {
        osirs::println!("Line {}", LineIndex);
    }
}