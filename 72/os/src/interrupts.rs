use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref Idt: InterruptDescriptorTable = {
        let mut Table = InterruptDescriptorTable::new();
        Table.breakpoint.set_handler_fn(BreakpointHandler);
        Table
    };
}

pub fn InitIdt() {
    Idt.load();
}

extern "x86-interrupt" fn BreakpointHandler(_StackFrame: InterruptStackFrame) {}