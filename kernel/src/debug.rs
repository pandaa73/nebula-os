use core::fmt;

use spin::Mutex;
use lazy_static::lazy_static;

const COM1: usize = 0x3F8;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    DEBUGGER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref DEBUGGER: Mutex<Writer> = Mutex::new(Writer {}.init());
}

pub struct Writer;

// NOTE: Maybe remove debugging in release mode?
impl Writer {
    fn init(self) -> Writer {
        self.outb(COM1 + 1, 0x00); // Disable interrupts
        self.outb(COM1 + 3, 0x80); // Enable DLAB
        self.outb(COM1 + 0, 0x03); // Baud rate divisor (lo byte)
        self.outb(COM1 + 1, 0x00); //                   (hi byte)
        self.outb(COM1 + 3, 0x03); // 8 bits, no pairity, one stop bit
        self.outb(COM1 + 2, 0x01); // Enable FIFO buffers
        self.outb(COM1 + 4, 0x00);

        self
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_byte(&self, byte: u8) {
        while !self.is_transmit_empty() {}

        self.outb(COM1 + 0, byte);
    }

    fn is_transmit_empty(&self) -> bool {
        (self.inb(COM1 + 5) & 0x20u8) != 0u8
    }

    #[inline(always)]
    fn outb(&self, port: usize, byte: u8) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") port,
                in("al") byte,
                options(nomem, nostack, preserves_flags)
            );
        }
    }

    #[inline(always)]
    fn inb(&self, port: usize) -> u8 {
        let byte: u8;
        unsafe {
            core::arch::asm!(
                "in al, dx",
                in("dx") port,
                out("al") byte,
                options(nomem, nostack, preserves_flags)
            );
        }

        byte
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
