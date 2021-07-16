use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {});
}

pub struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            outb(0x3F8, byte);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

pub unsafe fn outb(port: u16, val: u8) {
    llvm_asm!("outb $0, $1" : : "{al}"(val), "{dx}N"(port));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
