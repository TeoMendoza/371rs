#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::TestRunner)]
#![reexport_test_harness_main = "test_main"]

pub mod clock;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

use core::panic::PanicInfo;
use core::sync::atomic::AtomicU64;

pub const QemuPass: u32 = 0xA;
pub const QemuFail: u32 = 0xB;
pub static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

pub fn Init() {
    gdt::Init();
    interrupts::InitIdt();
}

pub fn HltLoop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn QemuQuit(QemuCode: u32) -> ! {
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(QemuCode);
    }

    loop {}
}

pub fn TestPanicHandler(PanicInformation: &PanicInfo) -> ! {
    serial_println!("[Failed]\nError: {}\n", PanicInformation);
    QemuQuit(QemuFail);
}

pub fn TestRunner(Tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", Tests.len());

    for Test in Tests {
        Test();
        serial_println!("Test In Test Runner Passed");
    }

    QemuQuit(QemuPass);
}

#[cfg(test)]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    TestPanicHandler(PanicInformation)
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    Init();
    test_main();
    QemuQuit(QemuPass);
}