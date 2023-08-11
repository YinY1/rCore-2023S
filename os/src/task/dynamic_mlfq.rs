use super::TaskControlBlock;
use alloc::{collections::VecDeque, sync::Arc, vec, vec::Vec};

/// Raisable & lowerable Multi-level Feedback Queue.
///
/// 1. If pa.prio > pb.prio, run pa
///
/// 2. If pa.prio = pb.prio, round-robin run pa & pb
///
/// 3. When a new forked process is first ready, set to the highest priority
///
/// 4. When a process runs out of its time quota, lower its priority
///
/// 5. Periodically count the waiting time of a process in the ready/blocked state,
/// and the longer the waiting time, the higher the priority of the process
#[allow(dead_code)]
pub struct FeedbackQueue {
    queues: Mlfq,
    timer: usize,
}

type ReadyTime = usize;
type Tcb = Arc<TaskControlBlock>;
type Mlfq = Vec<VecDeque<(Tcb, ReadyTime)>>;

const PRIORITY_LEVEL: usize = 16;
const TIMER_LIMIT: usize = 10;

#[allow(unused)]
impl FeedbackQueue {
    pub fn new() -> Self {
        FeedbackQueue {
            queues: vec![VecDeque::new(); PRIORITY_LEVEL],
            timer: 0,
        }
    }

    /// add a new task to queue with hightest priority
    pub fn add(&mut self, task: Tcb) {
        task.inner_exclusive_access().priority = 0;
        self.queues[0].push_back((task, 0));
    }

    /// fetch a task with highest priority
    pub fn fetch(&mut self) -> Option<Tcb> {
        for q in &mut self.queues {
            if !q.is_empty() {
                return q.pop_front().map(|(task, _)| task);
            }
        }
        None
    }

    /// after running out of current time, low the task priority and re-add the task to queue
    pub fn low(&mut self, task: Tcb) {
        let mut inner = task.inner_exclusive_access();
        if inner.priority < PRIORITY_LEVEL - 1 {
            inner.priority += 1;
        }
        let prio = inner.priority;
        drop(inner);
        self.queues[prio].push_back((task, 0));
    }

    fn calculate_ready_time(&self) -> Option<(usize, usize)> {
        let mut shortest: Option<usize> = None;
        let mut longest: Option<usize> = None;
        for q in &self.queues {
            for (_, time) in q {
                if shortest.map_or(true, |min_key| *time < min_key) {
                    shortest = Some(*time);
                }
                if longest.map_or(true, |max_key| *time > max_key) {
                    longest = Some(*time);
                }
            }
        }
        shortest.zip(longest)
    }

    /// update priority of each ready task
    ///
    /// since there is no blocked status, simply treat ready status is ok
    fn update_priority(&mut self, shortest: usize, longest: usize) {
        let mut new_queues: Mlfq = vec![VecDeque::new(); PRIORITY_LEVEL];
        let delta = (longest - shortest) / (PRIORITY_LEVEL - 1);

        for queue in &self.queues {
            for (task, time) in queue {
                let mut inner = task.inner_exclusive_access();
                let upper_level = (time - shortest).div_ceil(delta);

                let mut prio = 0; // highest priority
                if inner.priority > upper_level {
                    prio = inner.priority - upper_level; // raise priority
                }
                inner.priority = prio;
                drop(inner);
                new_queues[prio].push_back((task.clone(), *time));
            }
        }

        self.queues = new_queues;
    }

    pub fn check_priority(&mut self) {
        self.timer += 1;
        if self.timer == TIMER_LIMIT {
            self.timer = 0;
            if let Some((shortest, longest)) = self.calculate_ready_time() {
                self.update_priority(shortest, longest);
            }
        }
    }
}

#[allow(dead_code)]
struct TaskManagerMlfq {
    ready_queue: FeedbackQueue,
}

#[allow(dead_code)]
impl TaskManagerMlfq {
    pub fn new() -> Self {
        TaskManagerMlfq {
            ready_queue: FeedbackQueue::new(),
        }
    }

    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.add(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.fetch()
    }

    /// use in suspend and run next task
    ///
    /// every TIMER_LIMIT suspend updates all task priorities once
    pub fn check_priority(&mut self) {
        self.ready_queue.check_priority();
    }
}
