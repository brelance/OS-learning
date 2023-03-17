//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

const SYSCALL_TASK_INFO: usize = 410;

use core::usize;

use crate::task::set_syscall_info;

/// system call infromation
#[derive(Clone, Copy)]
pub struct SyscallInfo {
    /// id: system call id
    pub id: usize,
    /// times: system call times
    pub times: usize,
}

impl SyscallInfo {
    /// initialize syscall_info id
    pub fn with_id(&mut self, id: usize) {
        match id {
            0 => self.id = SYSCALL_WRITE,
            1 => self.id = SYSCALL_EXIT,
            2 => self.id = SYSCALL_YIELD,
            3 => self.id = SYSCALL_GET_TIME,
            4 => self.id = SYSCALL_TASK_INFO,
            _ => panic!("unreachable"),
        }
    }
    // fn with_id(&mut self, id: usize) {
    //     match id {
    //         => self.id = SYSCALL_WRITE,
    //         => self.id = SYSCALL_EXIT,
    //         => self.id = SYSCALL_YIELD,
    //         => self.id = SYSCALL_GET_TIME,
    //         => self.id = SYSCALL_TASK_INFO,
    //         _ => panic!("unreachable"),
    //     }
    // }
}

/// system call id.
#[derive(Clone, Copy)]
pub enum SyscallId {
    /// task_info.call[0]
    SyscallWrite,
    /// task_info.call[1] 
    SyscallExit,
    /// task_info.call[2] 
    SyscallYield,
    /// task_info.call[3] 
    SyscallGetTime,
    /// task_info.call[4] 
    SyscallTaskInfo,
    /// Max system call number 
    MaxTaskNum,
}





mod fs;
mod process;

use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => {
            set_syscall_info(SyscallId::SyscallWrite as usize);
            sys_write(args[0], args[1] as *const u8, args[2])
        }
        SYSCALL_EXIT => {
            set_syscall_info(SyscallId::SyscallExit as usize);
            sys_exit(args[0] as i32)
        }
        SYSCALL_YIELD => {
            set_syscall_info(SyscallId::SyscallYield as usize);
            sys_yield()
        }
        SYSCALL_GET_TIME => {
            set_syscall_info(SyscallId::SyscallGetTime as usize);
            sys_get_time()
        }
        SYSCALL_TASK_INFO => {
            set_syscall_info(SyscallId::SyscallTaskInfo as usize);
            sys_task_info(args[0], args[1] as *mut usize)
        }
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
