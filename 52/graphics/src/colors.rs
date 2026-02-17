use core::ptr::write_volatile;
mod img;

const Mmio: usize = 0xb8000;

const ScreenWidth: usize = 80;
const ScreenHeight: usize = 25;
const CellCount: usize = ScreenWidth * ScreenHeight;

pub fn colors() {
    let ColumnWidth = ScreenWidth / 16;

    unsafe {
        for Row in 0..ScreenHeight {
            for Col in 0..ScreenWidth {
                let Background = (Col / ColumnWidth) as u8;
                let Foreground = 0u8;
                let Attribute = (Background << 4) | Foreground;

                let Index = Row * ScreenWidth + Col;
                let Cell = (Mmio + (Index * 2)) as *mut u8;

                write_volatile(Cell, 32u8);
                write_volatile(Cell.add(1), Attribute);
            }
        }
    }
}

pub fn image() {
    unsafe {
        for Index in 0..CellCount {
            let Background = img::Img[Index] & 0x0f;
            let Foreground = 0u8;
            let Attribute = (Background << 4) | Foreground;

            let Cell = (Mmio + (Index * 2)) as *mut u8;
            write_volatile(Cell, 32u8);
            write_volatile(Cell.add(1), Attribute);
        }
    }
}