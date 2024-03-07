#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub trait Testable {
    fn run(&self);
}

// we implement our trait testable, and thus the `run` function, for functions
// the implementation is quite simple, we just add some extra printing and of course, run the desired function to test it.
impl<T: Fn()> Testable for T {
    fn run(&self) {
        // we are using serial_print so that everything is printed in the host machine,
        // and not in our development environment - that being QEMU
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
// we also want to create a separate panic handler with some nice printing to show when something fails.
#[allow(clippy::empty_loop)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    // when we want to indicate that a test fails (because it panics and thus calls this function),
    // we can signal that by exiting QEMU with a failure code
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

// accepts a `slice`(?) of functions that implement Testable to run and test! pretty simple
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests...", tests.len());
    for test in tests {
        test.run();
    }
    // if we make it through all of the tests, we can return from QEMU with success :)
    exit_qemu(QemuExitCode::Success);
}

// the entry point for the OS when testing
// indicated by the cfg(test)
#[cfg(test)]
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
// wrapper ?
// honestly forgot why we need this
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// safe wrapper enum for qemu exit codes
// useful for exiting qemu to indicate that tests succeed or fail
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
// exit qemu by writing the desired exit code to a port (??)
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
