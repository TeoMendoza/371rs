#![allow(non_snake_case)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(BreakpointHandler);
        unsafe {
            idt.double_fault
                .set_handler_fn(DoubleFaultHandler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn InitIdt() {
    IDT.load();
}

extern "x86-interrupt" fn BreakpointHandler(_stack_frame: InterruptStackFrame) {}

extern "x86-interrupt" fn DoubleFaultHandler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT");
}