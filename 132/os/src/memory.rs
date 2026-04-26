#![allow(non_snake_case)]

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr,
    VirtAddr,
    structures::paging::{
        FrameAllocator,
        Mapper,
        OffsetPageTable,
        Page,
        PageTable,
        PageTableFlags,
        PhysFrame,
        Size4KiB,
    },
};

pub unsafe fn ActiveLevel4Table(Offset: VirtAddr) -> &'static mut PageTable {
    let (Frame, _) = x86_64::registers::control::Cr3::read();
    let PhysicalAddress = Frame.start_address();
    let VirtualAddress = Offset + PhysicalAddress.as_u64();
    let PageTablePointer: *mut PageTable = VirtualAddress.as_mut_ptr();

    unsafe { &mut *PageTablePointer }
}

pub unsafe fn Init(Offset: VirtAddr) -> OffsetPageTable<'static> {
    let Level4Table = unsafe { ActiveLevel4Table(Offset) };
    unsafe { OffsetPageTable::new(Level4Table, Offset) }
}

pub fn CreateExampleMapping(
    Page: Page,
    Mapper: &mut OffsetPageTable<'static>,
    FrameAllocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let Frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let Flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    let MapToResult = unsafe { Mapper.map_to(Page, Frame, Flags, FrameAllocator) };
    MapToResult.expect("MapTo Failed").flush();
}

pub struct BootInfoFrameAllocator {
    MemoryMap: &'static MemoryMap,
    Next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn Init(MemoryMap: &'static MemoryMap) -> Self {
        Self {
            MemoryMap,
            Next: 0,
        }
    }

    fn UsableFrames(&self) -> impl Iterator<Item = PhysFrame> {
        let Regions = self.MemoryMap.iter();
        let UsableRegions = Regions.filter(|Region| Region.region_type == MemoryRegionType::Usable);
        let AddressRanges = UsableRegions.map(|Region| Region.range.start_addr()..Region.range.end_addr());
        let FrameAddresses = AddressRanges.flat_map(|Range| Range.step_by(4096));

        FrameAddresses.map(|Address| PhysFrame::containing_address(PhysAddr::new(Address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let Frame = self.UsableFrames().nth(self.Next);
        self.Next += 1;
        Frame
    }
}