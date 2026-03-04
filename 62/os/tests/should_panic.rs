#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::QemuQuit(osirs::QemuPass);
}

fn bad() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::TestRunner(&[&bad]);
    osirs::QemuQuit(osirs::QemuFail);
}