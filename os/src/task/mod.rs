//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the operating system.
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.

mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UPSafeCell;
use crate::task::switch::{print_switch_time, task_switch};
use crate::timer::get_time_ms;
use lazy_static::*;
use task::{TaskControlBlock, TaskInfo, TaskStatus};

pub use context::TaskContext;

/// The task manager, where all the tasks are managed.
///
/// Functions implemented on `TaskManager` deals with all task state transitions
/// and task context switching. For convenience, you can find wrappers around it
/// in the module level.
///
/// Most of `TaskManager` are hidden behind the field `inner`, to defer
/// borrowing checks to runtime. You can see examples on how to use `inner` in
/// existing functions on `TaskManager`.
pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    /// use inner value to get mutable access
    inner: UPSafeCell<TaskManagerInner>,
}

/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
    stop_time: usize,
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let task_info = TaskInfo::new();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_info,
            kernel_time: 0,
            user_time: 0,
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_info.status =TaskStatus::Ready;

            task.task_info.id = i;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                    stop_time: 0,
                })
            },
        }
    };
}

impl TaskManager {
    /// Run the first task in task list.
    ///
    /// Generally, the first task in task list is an idle task (we call it zero process later).
    /// But in ch3, we load apps statically, so the first task is a real app.
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_info.status = TaskStatus::Running;
        task0.task_info.time = get_time_ms();
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        let mut _unused = TaskContext::zero_init();

        // before this, we should drop local variables that must be dropped manually

        // unsafe {
        //     __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        // }

        inner.refresh_stop_time();
        drop(inner);
        unsafe {
            task_switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }

        panic!("unreachable in run_first_task!");
    }

    /// Change the status of current `Running` task into `Ready`.
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_info.status = TaskStatus::Ready;

        inner.tasks[current].kernel_time += inner.refresh_stop_time();

        println!("tasks[{}]: Running to Ready", current);
    }

    /// Change the status of current `Running` task into `Exited`.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_info.status = TaskStatus::Exited;

        inner.tasks[current].kernel_time += inner.refresh_stop_time();
        let start_time = inner.tasks[current].task_info.time;
        let exit_time = get_time_ms();

        inner.tasks[current].task_info.time = exit_time - start_time;

        println!(
            "tasks[{}] excited. user_time: {} ms, kernel_time: {} ms exec_time: {}",
            current,
            inner.tasks[current].user_time,
            inner.tasks[current].kernel_time,
            inner.tasks[current].task_info.time
        );
        drop(inner);
    }

    /// Find next task to run and return task id.
    ///
    /// In this case, we only return the first `Ready` task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_info.status == TaskStatus::Ready)
    }

    /// Switch current `Running` task to the task we have found,
    /// or there is no `Ready` task and we can exit with all applications completed
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[current].task_info.status = TaskStatus::Running;

            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            println!("tasks[{}]: Ready to Running", next);

            // before this, we should drop local variables that must be dropped manually
            // unsafe {
            //     __switch(current_task_cx_ptr, next_task_cx_ptr);
            // }

            inner.refresh_stop_time();

            if inner.tasks[next].task_info.time == 0 {
                inner.tasks[next].task_info.time = get_time_ms();
            }
            
            drop(inner);

            unsafe {
                task_switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            println!("All applications completed!");
            unsafe {
                print_switch_time();
            }
            use crate::board::QEMUExit;
            crate::board::QEMU_EXIT_HANDLE.exit_success();
        }
    }

    fn user_time_start(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.refresh_stop_time();
        drop(inner);
    }

    fn user_time_end(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].user_time += inner.refresh_stop_time();
        drop(inner);
    }

    fn set_syscall_info(&self, sys_info_id: usize) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_info.call[sys_info_id].times += 1;
        drop(inner);
    }

    fn get_task_info(&self, task_id: usize, ts: *mut TaskInfo) -> isize {
        if task_id > MAX_APP_NUM {
            println!("[kernel]: get_task_info task_id {} > max_app_num", task_id);
            return -1;
        }
        let inner = self.inner.exclusive_access();
        unsafe { ts.write(inner.tasks[task_id].task_info) };
        drop(inner);
        1
    }
}

impl TaskManagerInner {
    fn refresh_stop_time(&mut self) -> usize {
        let last_time = self.stop_time;
        self.stop_time = get_time_ms();
        self.stop_time - last_time
    }
}

/// run first task
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// rust next task
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// suspend current task
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// exit current task
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// suspend current task, then run next task
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// exit current task,  then run next task
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

/// user_time start point
pub fn user_time_start() {
    TASK_MANAGER.user_time_start();
}

/// user_time end point
pub fn user_time_end() {
    TASK_MANAGER.user_time_end();
}

/// set system call information in task_info
pub fn set_syscall_info(sys_info_id: usize) {
    TASK_MANAGER.set_syscall_info(sys_info_id);
}

/// get task information
pub fn get_task_info(task_id: usize, ts: *mut usize) -> isize {
    TASK_MANAGER.get_task_info(task_id, ts as *mut TaskInfo)
}
