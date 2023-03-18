//! Types related to task management

use super::TaskContext;
use crate::syscall::{SyscallId, SyscallInfo};

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    // pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info: TaskInfo,

    pub user_time: usize,
    pub kernel_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; SyscallId::MaxTaskNum as usize],
    pub time: usize,
}

impl TaskInfo {
    pub fn new() -> Self {
        let mut call = [SyscallInfo { id: 0, times: 0 }; SyscallId::MaxTaskNum as usize];
        for (id, info) in call.iter_mut().enumerate() {
            info.with_id(id);
        }
        Self {
            id: 0,
            status: TaskStatus::UnInit,
            call,
            time: 0,
        }
    }
}
