//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The task start time
    pub start_time: usize,
    /// The task context
    pub task_cx: TaskContext,
    /// The running time in user
    pub user_time: usize,
    /// The running time in kernel
    pub kernel_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
