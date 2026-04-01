#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(DoubleFaultHandler)
                .set_stack_index(osirs::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn InitTestIdt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn DoubleFaultHandler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    osirs::serial_println!("[ok]");
    osirs::QemuQuit(osirs::QemuPass);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::serial_print!("double::double_fault_test... ");
    osirs::gdt::Init();
    InitTestIdt();

    unsafe {
        core::ptr::write_volatile(0xdeadbeef as *mut u8, 42);
    }

    osirs::serial_println!("[failed]");
    osirs::QemuQuit(osirs::QemuFail);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osirs::TestPanicHandler(info)
}