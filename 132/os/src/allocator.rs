#![allow(non_snake_case)]

use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator,
        Mapper,
        Page,
        PageTableFlags,
        Size4KiB,
    },
};

pub const HEAP_START: usize = 0x_C371_0000;
pub const HEAP_SIZE: usize = 1 << 16;

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

pub fn InitHeap(
    Mapper: &mut impl Mapper<Size4KiB>,
    FrameAllocator: &mut impl FrameAllocator<Size4KiB>,
) -> Option<()> {
    let PageRange = {
        let HeapStart = VirtAddr::new(HEAP_START as u64);
        let HeapEnd = HeapStart + HEAP_SIZE - 1u64;
        let HeapStartPage = Page::containing_address(HeapStart);
        let HeapEndPage = Page::containing_address(HeapEnd);

        Page::range_inclusive(HeapStartPage, HeapEndPage)
    };

    let Flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for Page in PageRange {
        let Frame = match FrameAllocator.allocate_frame() {
            Some(Frame) => Frame,
            None => return None,
        };

        unsafe {
            match Mapper.map_to(Page, Frame, Flags, FrameAllocator) {
                Ok(Mapping) => Mapping.flush(),
                Err(_) => return None,
            }
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Some(())
}