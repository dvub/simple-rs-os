#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simple_rs_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use simple_rs_os::{init, println};
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // load idt
    init();

    // this will only be executed when testing
    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    simple_rs_os::test_panic_handler(info)
}
