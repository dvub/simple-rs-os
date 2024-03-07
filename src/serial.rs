use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        // initialize, wrap in a spinlock and we're good
        serial_port.init();

        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    instructions::interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    })
}

// custom macro black magic witchcraft bullshit that i don't understand
// this part is totally beyond me

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
