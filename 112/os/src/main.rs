#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::TestRunner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{
    VirtAddr,
    structures::paging::{Page, Translate},
};

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

    osirs::serial_println!("BootInfo: {:?}", BootInformation);

    let Addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        BootInformation.physical_memory_offset,
    ];

    for &Address in &Addresses {
        let VirtualAddress = VirtAddr::new(Address);
        let PhysicalAddress = Mapper.translate_addr(VirtualAddress);
        osirs::serial_println!("{:?} -> {:?}", VirtualAddress, PhysicalAddress);
    }

    let mut FrameAllocator =
        unsafe { osirs::memory::BootInfoFrameAllocator::Init(&BootInformation.memory_map) };

    let Page = Page::containing_address(VirtAddr::new(0));
    osirs::memory::CreateExampleMapping(Page, &mut Mapper, &mut FrameAllocator);

    let PageStart: *mut u8 = Page.start_address().as_mut_ptr();
    unsafe {
        PageStart.add(160).write_volatile(b'N');
        PageStart.add(161).write_volatile(0x0f);
        PageStart.add(162).write_volatile(b'e');
        PageStart.add(163).write_volatile(0x0f);
        PageStart.add(164).write_volatile(b'w');
        PageStart.add(165).write_volatile(0x0f);
        PageStart.add(166).write_volatile(b'!');
        PageStart.add(167).write_volatile(0x0f);
    }

    osirs::clock::PrintPrompt();
    osirs::vga::WriteAt(1, 0, "New!");

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    loop {
        osirs::clock::Update();
        x86_64::instructions::hlt();
    }

    #[cfg(test)]
    loop {}
}