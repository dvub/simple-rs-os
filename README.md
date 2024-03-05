# Simple-rs-OS

This project is a very simple operating system built in Rust. At the moment, I'm building this project following [this fantastic article](https://os.phil-opp.com/) on osdev in Rust (since I have no idea what I'm doing, obviously).

## How to Build

This section is for anyone who would like to build this OS from source for themselves. You'll need to make sure to:

- Use the **nightly** compiler (to change just the CD, use `rustup override set nightly`)
- Install the LLVM-tools component (use `rustup component add llvm-tools-preview`)
- Install the `bootimage` tool (In your home directory, run `cargo install bootimage`)
- Install QEMU (on arch linux, for me, I use `qemu-desktop`)

Once that's all done, you can use `cargo build` to build the OS.
