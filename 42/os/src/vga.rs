use core::ptr::{read_volatile, write_volatile};

static mut LATEST: usize = 0;

const MMIO: usize = 0xb8000;
const COLOR: u8 = 0x0f;

const ROWS: usize = 80;
const COLS: usize = 25;
const MAX: usize = ROWS * COLS;

pub fn char_to_vga(a: u8) {
    unsafe {
        if LATEST >= MAX {
            scroll();
        }

        let cell: *mut u8 = (MMIO + (LATEST * 2)) as *mut u8;

        write_volatile(cell, a);
        write_volatile(cell.add(1), COLOR);

        LATEST += 1;
    }
}

pub fn str_to_vga(s: &str) {
    let bytes = s.as_bytes();

    unsafe {
        for i in 0..bytes.len() {
            if LATEST >= MAX {
                scroll();
            }

            match bytes[i] {
                10 => {
                    LATEST = ((LATEST / ROWS) + 1) * ROWS;
                }
                other => {
                    char_to_vga(other);
                }
            }
        }
    }
}

pub fn scroll() {
    unsafe {
        for i in ROWS..MAX {
            let src: *mut u8 = (MMIO + i * 2) as *mut u8;
            let dst: *mut u8 = (MMIO + (i - ROWS) * 2) as *mut u8;

            let ch = read_volatile(src);
            write_volatile(dst, ch);
            write_volatile(dst.add(1), COLOR);
        }

        for i in (MAX - ROWS)..MAX {
            let dst: *mut u8 = (MMIO + i * 2) as *mut u8;
            write_volatile(dst, 32);
            write_volatile(dst.add(1), COLOR);
        }

        if LATEST >= ROWS {
            LATEST -= ROWS;
        } else {
            LATEST = 0;
        }
    }
}
