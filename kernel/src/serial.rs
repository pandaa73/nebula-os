// NOTE: Maybe remove debugging in release mode?

use core::fmt;

use spin::Mutex;
use lazy_static::lazy_static;

const DBG_PORT: usize = 0x3F8; // COM1 for QEMU

/// Macro for debug printing to the COM1 port
#[macro_export]
macro_rules! dbg_print {
    ($($arg:tt)*) => ($crate::serial::_dbg_print(format_args!($($arg)*)));
}

/// Macro for debug printing to the COM1 port; appends a newline
#[macro_export]
macro_rules! dbg_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::dbg_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _dbg_print(args: fmt::Arguments) {
    use core::fmt::Write;
    DEBUGGER.lock().write_fmt(args).unwrap();
}

lazy_static! {
    pub static ref DEBUGGER: Mutex<SerialWriter> = Mutex::new(SerialWriter {
        port: DBG_PORT,
    }.init());
}

/// A writer for COM serial ports
pub struct SerialWriter {
    port: usize,
}

impl SerialWriter {
    fn init(self) -> SerialWriter {
        self.outb(self.port + 1, 0x00); // Disable interrupts
        self.outb(self.port + 3, 0x80); // Enable DLAB
        self.outb(self.port + 0, 0x03); // Baud rate divisor (lo byte)
        self.outb(self.port + 1, 0x00); //                   (hi byte)
        self.outb(self.port + 3, 0x03); // 8 bits, no pairity, one stop bit
        self.outb(self.port + 2, 0x01); // Enable FIFO buffers
        self.outb(self.port + 4, 0x00);

        self
    }

    /// Writes a string to the serial port
    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    /// Writes a byte to the serial port
    pub fn write_byte(&self, byte: u8) {
        while !self.is_transmit_empty() {}

        self.outb(self.port + 0, byte);
    }

    fn is_transmit_empty(&self) -> bool {
        (self.inb(self.port + 5) & 0x20u8) != 0u8
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

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
