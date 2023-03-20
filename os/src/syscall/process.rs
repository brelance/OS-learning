use crate::mm::{write_bytes_buffer, mmap};
use crate::task::{current_user_token, exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let time = TimeVal {
        sec: _us / 1_000_000,
        usec: _us % 1_000_000,
    };
    write_bytes_buffer(
        current_user_token(),
        _ts as usize as *mut u8,
        &time as *const _ as *const u8,
        16,
    )
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    mmap(current_user_token(), start, len, prot)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    0
}
