#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::task_info;
use user_lib::task::*;

const MAX_APP_NUM: usize = 5;

#[no_mangle]
fn main() -> isize {
    println!("=== taskinfo test start ===");
    let mut taskinfo = TaskInfo {
        id: 0,
        status: TaskStatus::UnInit,
        call: [SyscallInfo { id: 0, times: 0 }; MAX_APP_NUM],
        time: 0,
    };

    task_info(3, &mut taskinfo);

    println!("task_info: {:#?}",  taskinfo);

    println!("=== taskinfo test end ===");
    1

}
