#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

mod vga;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("It is I, a panic!");
    loop {}
}

#[panic_handler]
fn panic(Info: &PanicInfo) -> ! {
    if let Some(Location) = Info.location() {
        println!("PANIC at {}:{}", Location.file(), Location.line());
    } else {
        println!("PANIC at <unknown>");
    }

    print!("Message: ");
    print!("{}", Info.message());
    println!();

    loop {}
}

