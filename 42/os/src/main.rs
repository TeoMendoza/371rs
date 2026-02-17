#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

mod vga;
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga::str_to_vga("Hello, world!\n");
    vga::str_to_vga("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\nA\nB\n");
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

