#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    _start()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}