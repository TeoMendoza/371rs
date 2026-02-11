#![no_std]
#![no_main]
#![allow(unconditional_recursion)]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    let VgaBufferAddress = 0xb8000 as *mut u8;
    let MessageBytes = b"Hello, world!";
    let ColorByte = 0x0f;

    for Index in 0..MessageBytes.len() {
        unsafe {
            VgaBufferAddress.add(Index * 2).write_volatile(MessageBytes[Index]);
            VgaBufferAddress.add(Index * 2 + 1).write_volatile(ColorByte);
        }
    }
    
    loop {} 
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic(info)
}

