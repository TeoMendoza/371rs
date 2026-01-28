// Treat ourselves to a kb (1024 bits)
// 1024 >> 3 == 128 == 0x80
pub const SIZE: usize = 0x80;
pub const MASK_BYTES: usize = SIZE >> 3;
pub const RESERVED_MASK_BYTES: usize = MASK_BYTES >> 3;
static mut BUS: [u8; SIZE] = [0u8; SIZE];

pub fn malloc(s: usize) -> Option<usize> {
    unsafe {

        if s == 0 {
            return None;
        }

        if BUS[0] != 0xFF {
            init();
        }

        let size_to_reserve = if s <= 8 { s } 
        else {
            let remainder = s % 8;
            if remainder == 0 { s } else { s + (8 - remainder) }
        };

        let step_size = if size_to_reserve <= 8 { 1 } else { 8 };

        for candidate_start_byte_index in (MASK_BYTES..= SIZE - size_to_reserve).step_by(step_size) {
            let mut is_free = true;

            for byte_index in candidate_start_byte_index..candidate_start_byte_index + size_to_reserve {
                let mask_byte_index = byte_index / 8;
                let mask_bit_index = byte_index % 8;

                if (BUS[mask_byte_index] & (1 << mask_bit_index)) != 0 {
                    is_free = false;
                    break;
                }
            }

            if is_free {
                for byte_index in candidate_start_byte_index..candidate_start_byte_index + size_to_reserve {
                    let mask_byte_index = byte_index / 8;
                    let mask_bit_index = byte_index % 8;

                    BUS[mask_byte_index] |= 1 << mask_bit_index;
                }

                return Some(candidate_start_byte_index);
            }
        }

        None
    }
}

pub fn setter(value: usize, index: usize) {

}

pub fn getter(index: usize) //-> i32 
{
    
}

// Zero the array except the mask.
fn init() {
  unsafe {
    assert!(SIZE & (SIZE - 1) == 0);

    for index in 0..RESERVED_MASK_BYTES {
        BUS[index] = 0xFF;
    }

    for index in RESERVED_MASK_BYTES..SIZE {
        BUS[index] = 0;
    }
  }
  return;
}