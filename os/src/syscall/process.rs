//! App management syscalls
use crate::batch::{run_next_app, get_taskinfo};


/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_get_taskinfo(task_info: *mut usize) -> isize {
    println!("system_call: [sys_get_taskinfo]");
    unsafe { get_taskinfo(task_info) }
}