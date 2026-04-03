#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut Idt = InterruptDescriptorTable::new();

        unsafe {
            Idt.double_fault
                .set_handler_fn(TestDoubleFaultHandler)
                .set_stack_index(osirs::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        Idt
    };
}

pub fn InitTestIdt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn TestDoubleFaultHandler(_StackFrame: InterruptStackFrame, _ErrorCode: u64) -> ! {
    osirs::serial_println!("[Pass]");
    osirs::QemuQuit(osirs::QemuPass);
}

#[allow(unconditional_recursion)]
fn StackOverflow() {
    StackOverflow();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::serial_println!("Double Fault Test Started");

    osirs::gdt::Init();
    InitTestIdt();

    StackOverflow();

    osirs::QemuQuit(osirs::QemuFail);
}