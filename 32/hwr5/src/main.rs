#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {} 
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}



// cargo build --target riscv64imac-unknown-none-elf

// qemu-system-riscv64 -machine sifive_u -bios none -nographic -kernel target/riscv64imac-unknown-none-elf/debug/osirs

