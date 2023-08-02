//! Process management syscalls
use crate::task::{map_current_with_len, unmap_current_with_len};
#[allow(unused)]
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{map_with_len, VirtAddr},
    task::{
        change_program_brk, exit_current_and_run_next, get_current_phys_addr,
        get_current_running_time_ms, get_current_status, get_current_syscall_times,
        suspend_current_and_run_next, TaskStatus,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let tv = get_current_phys_addr(ts as *const u8) as *mut TimeVal;
    unsafe {
        *tv = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let ti = get_current_phys_addr(ti as *const u8) as *mut TaskInfo;
    unsafe {
        *ti = TaskInfo {
            status: get_current_status(),
            syscall_times: get_current_syscall_times(),
            time: get_current_running_time_ms(),
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    let va = VirtAddr::from(start);
    if va.page_offset() != 0 || port & !0x7 != 0 || port & 0x7 == 0 {
        return -1; // illegal
    }
    map_current_with_len(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let va = VirtAddr::from(start);
    if va.page_offset() != 0 {
        return -1; // illegal
    }
    unmap_current_with_len(start, len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
