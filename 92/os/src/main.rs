#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::serial_println!("{}", PanicInformation);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::Init();
    osirs::vga::ClearScreen();
    osirs::clock::PrintPrompt();

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    loop {
        osirs::clock::Update();
        x86_64::instructions::hlt();
    }

    #[cfg(test)]
    loop {}
}