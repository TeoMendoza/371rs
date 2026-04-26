#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(TestMain);

#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

fn TestMain(BootInformation: &'static BootInfo) -> ! {
    osirs::Init();

    let PhysicalMemoryOffset = VirtAddr::new(BootInformation.physical_memory_offset);
    let mut Mapper = unsafe { osirs::memory::Init(PhysicalMemoryOffset) };
    let mut FrameAllocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::Init(&BootInformation.memory_map) };

    osirs::allocator::InitHeap(&mut Mapper, &mut FrameAllocator)
        .expect("Heap initialization failed");

    test_main();

    osirs::QemuQuit(osirs::QemuPass);
}

#[test_case]
fn SimpleAllocation() {
    let HeapValue1 = alloc::boxed::Box::new(41);
    let HeapValue2 = alloc::boxed::Box::new(13);

    assert_eq!(*HeapValue1, 41);
    assert_eq!(*HeapValue2, 13);
}

#[test_case]
fn LargeVec() {
    let Count = 1000;
    let mut Values = alloc::vec::Vec::new();

    for Index in 0..Count {
        Values.push(Index);
    }

    assert_eq!(Values.iter().sum::<u64>(), (Count - 1) * Count / 2);
}

#[test_case]
fn ManyBoxes() {
    for Index in 0..10000 {
        let Value = alloc::boxed::Box::new(Index);
        assert_eq!(*Value, Index);
    }
}