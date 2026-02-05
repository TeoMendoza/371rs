How To Set Up And Run On Emulated RISC-V On QEMU For MacOS

1. Install QEMU - brew install qemu
2. Install Rust Core Libraries For The RISC-V Bare Metal Target - rustup target add riscv64imac-unknown-none-elf
3. Clone Repo With Linkers - git clone https://github.com/cd-rs/hwr5.git
4. cd Into Repo - cd hwr5
5. Build Binary For Taret - cargo build --target riscv64imac-unknown-none-elf
6. Run Binary On QEMU - qemu-system-riscv64 -machine sifive_u -bios none -nographic -kernel target/riscv64imac-unknown-none-elf/debug/<'crate_name'>
7. Exit QEMU - Ctrl + A -> X