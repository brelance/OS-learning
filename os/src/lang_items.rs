//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use super::tools::stack_trace;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[kernel] Panicked: {}", info.message().unwrap());
        unsafe { stack_trace(); }
    }
    shutdown()
}
