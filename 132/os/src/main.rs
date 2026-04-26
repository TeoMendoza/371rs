#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

entry_point!(KernelMain);

#[cfg(not(test))]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::serial_println!("{}", PanicInformation);
    osirs::HltLoop()
}

#[cfg(test)]
#[panic_handler]
fn panic(PanicInformation: &PanicInfo) -> ! {
    osirs::TestPanicHandler(PanicInformation)
}

fn KernelMain(BootInformation: &'static BootInfo) -> ! {
    osirs::Init();

    let PhysicalMemoryOffset = VirtAddr::new(BootInformation.physical_memory_offset);
    let mut Mapper = unsafe { osirs::memory::Init(PhysicalMemoryOffset) };

    let mut FrameAllocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::Init(&BootInformation.memory_map) };

    osirs::allocator::InitHeap(&mut Mapper, &mut FrameAllocator)
        .expect("Heap initialization failed");

    osirs::snake::Init();

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    loop {
        osirs::snake::Update();
        x86_64::instructions::hlt();
    }

    #[cfg(test)]
    loop {}
}