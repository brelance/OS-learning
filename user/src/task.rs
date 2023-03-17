

#[derive(Debug)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; SyscallId::MaxTaskNum as usize],
    pub time: usize,
}

#[derive(Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Clone, Copy, Debug)]
pub struct SyscallInfo {
    /// id: system call id
    pub id: usize,
    /// times: system call times
    pub times: usize,
}

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