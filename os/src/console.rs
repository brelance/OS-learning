//! SBI console driver, for text output

use crate::sbi::console_putchar;
use core::fmt::{self, Write};
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/// error log
#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    }
}

/// warn log
#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    }
}

/// info log
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    }
}

/// trace log
#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[90m", $fmt, "\x1b[0m", "\n") $(, $($arg)+)?));
    }
}
