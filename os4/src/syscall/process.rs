//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::mm::{MapPermission, VirtAddr};
use crate::task::{current_user_token, exit_current_and_run_next, map_address_current_task, suspend_current_and_run_next, TASK_MANAGER, TaskStatus, unmap_address_current_task};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    // need align
    let start_va: VirtAddr = _start.into();
    if !start_va.aligned() {
        return -1;
    }

    let end_va: VirtAddr = (_start + _len).into();
    if let Some(perm) = change_port_to_permission(_port) {
        trace!("Map start: {:#x} to end: {:#x}", start_va.0, end_va.0);

        if let Some(()) = map_address_current_task(start_va, end_va, perm) {
            0
        }else {
            -1
        }
    }else {
        -1
    }
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    let start_va: VirtAddr = _start.into();
    if !start_va.aligned() {
        return -1;
    }
    let end_va: VirtAddr = (_start + _len).into();
    if let Some(()) = unmap_address_current_task(start_va, end_va) {
        return 0;
    }
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    -1
}

fn change_port_to_permission(port: usize) -> Option<MapPermission> {
    let user_permission = MapPermission::U;
    let (read, write, execute) = (MapPermission::R, MapPermission::W, MapPermission::X);
    let permission = match port {
        1 => Some(read | user_permission),
        2 => Some(write | user_permission),
        4 => Some(execute | user_permission),
        3 => Some(read | write | user_permission),
        5 => Some(read | execute | user_permission),
        6 => Some(write | execute | user_permission),
        7 => Some(write | execute | read | user_permission),
        _ => None
    };
    permission
}