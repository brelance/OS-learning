#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::get_taskinfo;

fn main() {
    let mut task_info: [usize; 4] = [0; 4];
    get_taskinfo(task_info.as_mut_ptr());
    println!(
        "[get_taskinfo]: current_task_id: {} task_addr: [{:#x}, {:#x})",
        task_info[0], task_info[1], task_info[2]
    );
    println!("test end");
}
