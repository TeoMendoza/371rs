#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::TestRunner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod serial;
pub mod interrupts;

use core::panic::PanicInfo;

pub const QemuPass: u32 = 0xA;
pub const QemuFail: u32 = 0xB;

pub fn Init() {
    interrupts::InitIdt();
}

pub fn QemuQuit(QemuCode: u32) -> ! {
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(QemuCode);
    }
    loop {}
}

pub fn TestPanicHandler(PanicInformation: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", PanicInformation);
    QemuQuit(QemuFail);
}

pub fn TestRunner(Tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", Tests.len());
    for Test in Tests {
        Test();
        serial_println!("[ok]");
    }
    QemuQuit(QemuPass);
}

#[cfg(test)]
#[panic_handler]
fn Panic(PanicInformation: &PanicInfo) -> ! {
    TestPanicHandler(PanicInformation)
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Init();
    test_main();
    QemuQuit(QemuPass);
}