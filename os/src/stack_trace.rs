//! stack trace

use core::{arch::asm, ptr};

/// print stack trace
pub unsafe fn stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("=== stack trace start ===");
        println!("ra: 0x{:016x}, fp: 0x{:016x}", saved_ra, saved_fp);
        
        fp = saved_fp as *const usize;
    }
    println!("=== stack trace end ===");
}