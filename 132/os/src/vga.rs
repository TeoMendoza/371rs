use core::fmt;
use core::ptr::{read_volatile, write_volatile};

use lazy_static::lazy_static;
use spin::Mutex;

const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x0f;

pub const ROWS: usize = 25;
pub const COLS: usize = 80;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub struct Writer {
    RowPosition: usize,
    ColumnPosition: usize,
    ColorCode: u8,
}

impl Writer {
    pub const fn new() -> Self {
        Self {
            RowPosition: 0,
            ColumnPosition: 0,
            ColorCode: COLOR,
        }
    }

    fn write_byte(&mut self, Byte: u8) {
        match Byte {
            b'\n' => self.new_line(),
            Byte => {
                if self.ColumnPosition >= COLS {
                    self.new_line();
                }

                self.write_cell(self.RowPosition, self.ColumnPosition, Byte, self.ColorCode);
                self.ColumnPosition += 1;
            }
        }
    }

    fn write_string(&mut self, Text: &str) {
        for Byte in Text.bytes() {
            match Byte {
                0x20..=0x7e | b'\n' => self.write_byte(Byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.ColumnPosition = 0;

        if self.RowPosition < ROWS - 1 {
            self.RowPosition += 1;
        } else {
            self.scroll_up();
            self.clear_row(ROWS - 1);
        }
    }

    fn scroll_up(&mut self) {
        for RowIndex in 1..ROWS {
            for ColIndex in 0..COLS {
                let Character = self.read_character(RowIndex, ColIndex);
                let ColorCode = self.read_color(RowIndex, ColIndex);
                self.write_cell(RowIndex - 1, ColIndex, Character, ColorCode);
            }
        }
    }

    fn clear_row(&mut self, RowIndex: usize) {
        for ColIndex in 0..COLS {
            self.write_cell(RowIndex, ColIndex, b' ', self.ColorCode);
        }
    }

    fn read_character(&self, RowIndex: usize, ColIndex: usize) -> u8 {
        let Cell = self.cell_ptr(RowIndex, ColIndex);
        unsafe { read_volatile(Cell) }
    }

    fn read_color(&self, RowIndex: usize, ColIndex: usize) -> u8 {
        let Cell = self.cell_ptr(RowIndex, ColIndex);
        unsafe { read_volatile(Cell.add(1)) }
    }

    fn write_cell(&self, RowIndex: usize, ColIndex: usize, Character: u8, ColorCode: u8) {
        let Cell = self.cell_ptr(RowIndex, ColIndex);
        unsafe {
            write_volatile(Cell, Character);
            write_volatile(Cell.add(1), ColorCode);
        }
    }

    fn cell_ptr(&self, RowIndex: usize, ColIndex: usize) -> *mut u8 {
        let Offset = (RowIndex * COLS + ColIndex) * 2;
        (MMIO + Offset) as *mut u8
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, Text: &str) -> fmt::Result {
        self.write_string(Text);
        Ok(())
    }
}

pub fn _print(Arguments: fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(Arguments).unwrap();
    });
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

pub fn ClearScreen() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut Writer = WRITER.lock();

        for RowIndex in 0..ROWS {
            Writer.clear_row(RowIndex);
        }

        Writer.RowPosition = 0;
        Writer.ColumnPosition = 0;
    });
}

pub fn WriteAt(Row: usize, Col: usize, Text: &str) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut Writer = WRITER.lock();

        let OriginalRow = Writer.RowPosition;
        let OriginalCol = Writer.ColumnPosition;

        Writer.RowPosition = Row;
        Writer.ColumnPosition = Col;

        Writer.write_string(Text);

        Writer.RowPosition = OriginalRow;
        Writer.ColumnPosition = OriginalCol;
    });
}

pub fn WriteByteAt(Row: usize, Col: usize, Byte: u8) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let Writer = WRITER.lock();
        Writer.write_cell(Row, Col, Byte, COLOR);
    });
}

pub fn DisableCursor() {
    unsafe {
        let mut CommandPort = x86_64::instructions::port::Port::<u8>::new(0x3D4);
        let mut DataPort = x86_64::instructions::port::Port::<u8>::new(0x3D5);

        CommandPort.write(0x0A);
        DataPort.write(0x20);
    }
}

pub fn SetCursorPosition(Row: usize, Col: usize) {
    let Position: u16 = (Row * COLS + Col) as u16;

    unsafe {
        let mut CommandPort = x86_64::instructions::port::Port::<u8>::new(0x3D4);
        let mut DataPort = x86_64::instructions::port::Port::<u8>::new(0x3D5);

        CommandPort.write(0x0F);
        DataPort.write((Position & 0x00FF) as u8);

        CommandPort.write(0x0E);
        DataPort.write((Position >> 8) as u8);
    }
}

pub fn SetWriterPosition(Row: usize, Col: usize) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut Writer = WRITER.lock();
        Writer.RowPosition = Row;
        Writer.ColumnPosition = Col;
    });

    SetCursorPosition(Row, Col);
}