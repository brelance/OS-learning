//! Rust wrapper around `__switch`.
//!
//! Switching to a different task's context happens here. The actual
//! implementation must not be in Rust and (essentially) has to be in assembly
//! language (Do you know why?), so this module really is just a wrapper around
//! `switch.S`.

use super::TaskContext;
use core::arch::global_asm;
use crate::timer::get_time_us;

static mut SWITCH_TIME_START: usize = 0;
static mut SWITCH_TIME_COUNTRE: usize = 0;

global_asm!(include_str!("switch.S"));

extern "C" {
    /// Switch to the context of `next_task_cx_ptr`, saving the current context
    /// in `current_task_cx_ptr`.
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

pub unsafe fn print_switch_time() {
    println!("[switch-time]: {} us", SWITCH_TIME_COUNTRE);
}

pub unsafe fn task_switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *mut TaskContext) {
    SWITCH_TIME_START = get_time_us();
    __switch(current_task_cx_ptr, next_task_cx_ptr);
    SWITCH_TIME_COUNTRE += get_time_us() - SWITCH_TIME_START;
}



