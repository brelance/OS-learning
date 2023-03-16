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

/// syscall_id: get_taskinfo
const SYSCALL_GETTINFO: usize = 144;

static mut SYSTEMCALL_COUTER: [usize; 64] = [0; 64];

enum SyscallId {
    SysWrite,
    SysExit,
    SysGetinfo,
}

impl SyscallId {}

mod fs;
mod process;

use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub unsafe fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => {
            SYSTEMCALL_COUTER[SyscallId::SysWrite as usize] += 1;
            sys_write(args[0], args[1] as *const u8, args[2])
        }
        SYSCALL_GETTINFO => {
            SYSTEMCALL_COUTER[SyscallId::SysGetinfo as usize] += 1;
            sys_get_taskinfo(args[0] as *mut usize)
        }

        SYSCALL_EXIT => {
            SYSTEMCALL_COUTER[SyscallId::SysExit as usize] += 1;
            sys_exit(args[0] as i32)
        }

        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

/// print the count of each system call
pub unsafe fn print_syscall_count() {
    println!(
        "[syscall_counter]: SysWrite {} times",
        SYSTEMCALL_COUTER[SyscallId::SysWrite as usize]
    );
    println!(
        "[syscall_counter]: SysWrite {} times",
        SYSTEMCALL_COUTER[SyscallId::SysGetinfo as usize]
    );

    println!(
        "[syscall_counter]: SysExit {} times",
        SYSTEMCALL_COUTER[SyscallId::SysExit as usize]
    );
}
