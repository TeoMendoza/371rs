#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::println!("{}", PanicInformation);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::println!("Hello World!");
    osirs::Init();

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    {
        x86_64::instructions::interrupts::int3();
        osirs::println!("It did not crash!");
    }

    loop {}
}