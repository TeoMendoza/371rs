use core::fmt;
use core::ptr::{read_volatile, write_volatile};

static mut LATEST: usize = 0;

const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x0f;

const ROWS: usize = 80;
const COLS: usize = 25;
const MAX: usize = ROWS * COLS;

pub fn char_to_vga(Byte: u8) {
    unsafe {
        if LATEST >= MAX {
            scroll();
        }

        let Cell: *mut u8 = (MMIO + (LATEST * 2)) as *mut u8;

        write_volatile(Cell, Byte);
        write_volatile(Cell.add(1), COLOR);

        LATEST += 1;
    }
}

pub fn str_to_vga(Text: &str) {
    let Bytes = Text.as_bytes();

    unsafe {
        for Index in 0..Bytes.len() {
            if LATEST >= MAX {
                scroll();
            }

            match Bytes[Index] {
                10 => {
                    LATEST = ((LATEST / ROWS) + 1) * ROWS;
                }
                Other => {
                    char_to_vga(Other);
                }
            }
        }
    }
}

fn scroll() {
    unsafe {
        for Index in ROWS..MAX {
            let Source: *mut u8 = (MMIO + Index * 2) as *mut u8;
            let Destination: *mut u8 = (MMIO + (Index - ROWS) * 2) as *mut u8;

            let Character = read_volatile(Source);
            write_volatile(Destination, Character);
            write_volatile(Destination.add(1), COLOR);
        }

        for Index in (MAX - ROWS)..MAX {
            let Destination: *mut u8 = (MMIO + Index * 2) as *mut u8;
            write_volatile(Destination, 32);
            write_volatile(Destination.add(1), COLOR);
        }

        if LATEST >= ROWS {
            LATEST -= ROWS;
        } else {
            LATEST = 0;
        }
    }
}

pub struct Dummy {}

impl fmt::Write for Dummy {
    fn write_str(&mut self, Text: &str) -> fmt::Result {
        str_to_vga(Text);
        Ok(())
    }
}

pub fn _print(Arguments: fmt::Arguments) {
    use core::fmt::Write;
    let mut Writer = Dummy {};
    Writer.write_fmt(Arguments).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
