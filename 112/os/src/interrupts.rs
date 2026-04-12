#![allow(non_snake_case)]

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
    PageFaultErrorCode,
};

use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn AsU8(self) -> u8 {
        self as u8
    }

    fn AsUsize(self) -> usize {
        usize::from(self.AsU8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut Idt = InterruptDescriptorTable::new();

        Idt.breakpoint.set_handler_fn(BreakpointHandler);
        Idt.page_fault.set_handler_fn(PageFaultHandler);

        unsafe {
            Idt.double_fault
                .set_handler_fn(DoubleFaultHandler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        Idt[InterruptIndex::Timer.AsUsize()].set_handler_fn(TimerInterruptHandler);
        Idt[InterruptIndex::Keyboard.AsUsize()].set_handler_fn(KeyboardInterruptHandler);

        Idt
    };
}

pub fn InitIdt() {
    IDT.load();

    unsafe {
        PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn BreakpointHandler(_StackFrame: InterruptStackFrame) {
    crate::serial_println!("Breakpoint Exception Handled!");
}

extern "x86-interrupt" fn PageFaultHandler(StackFrame: InterruptStackFrame, ErrorCode: PageFaultErrorCode) {
    crate::println!("EXCEPTION: PAGE FAULT");
    crate::println!(
        "Accessed Address: {:?}",
        x86_64::registers::control::Cr2::read()
    );
    crate::println!("Error Code: {:?}", ErrorCode);
    crate::println!("{:#?}", StackFrame);

    crate::HltLoop();
}

extern "x86-interrupt" fn DoubleFaultHandler(StackFrame: InterruptStackFrame, _ErrorCode: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", StackFrame);
}

extern "x86-interrupt" fn TimerInterruptHandler(_StackFrame: InterruptStackFrame) {
    crate::TIMER_TICKS.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.AsU8());
    }
}

extern "x86-interrupt" fn KeyboardInterruptHandler(_StackFrame: InterruptStackFrame) {
    use lazy_static::lazy_static;
    use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ));
    }

    let mut Keyboard = KEYBOARD.lock();
    let mut Port60 = Port::new(0x60);
    let Scancode: u8 = unsafe { Port60.read() };

    if let Ok(Some(KeyEvent)) = Keyboard.add_byte(Scancode) {
        if let Some(Key) = Keyboard.process_keyevent(KeyEvent) {
            match Key {
                DecodedKey::Unicode(Character) => {
                    if crate::clock::IsInitialized() {
                        crate::print!("{}", Character);
                    } else {
                        crate::clock::HandleKey(Character);
                    }
                }
                DecodedKey::RawKey(_) => {}
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.AsU8());
    }
}