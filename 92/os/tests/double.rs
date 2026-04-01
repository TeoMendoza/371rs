#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_PanicInformation: &PanicInfo) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::QemuQuit(osirs::QemuPass);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::serial_println!("[double test start]");
    osirs::Init();
    
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    osirs::QemuQuit(osirs::QemuFail);
}