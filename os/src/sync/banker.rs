use alloc::collections::BTreeSet;

use alloc::vec;
use alloc::vec::Vec;

/// a struct to implement banker algorithm
pub struct Banker {
    /// size = m, `available[j] = k` means  has k j-th resource left
    pub available: Vec<usize>,
    /// size = n*m, `allocation[i][j] = g` means j-th resource of i-th thread has alloced g
    pub allocation: Vec<Vec<usize>>,
    /// size = n*m, `need[i][j] = d` means j-th resource of i-th thread still needs d
    pub need: Vec<Option<usize>>,
}

#[allow(unused)]
impl Banker {
    /// create a empty banker
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
        }
    }

    /// return total num mutex/semaphore created
    pub fn lock_num(&self) -> usize {
        self.available.len()
    }

    fn thread_num(&self) -> usize {
        self.allocation.len()
    }

    /// while creating a new thread, clear locks info if tid from recycled, or push something new;
    pub fn add_thread(&mut self, tid: usize) {
        if self.need.len() > tid {
            self.allocation[tid].clear();
            self.need[tid] = None;
        } else {
            let lock_num = self.lock_num();
            while self.need.len() <= tid {
                self.allocation.push(vec![0; lock_num]);
                self.need.push(None);
            }
        }
    }

    /// while creating a new lock from recycled, clear resources
    pub fn modify_lock(&mut self, tid: usize, lock_id: usize, res_count: usize) {
        self.available[lock_id] = res_count;
        self.allocation[tid][lock_id] = 0;
        self.need[tid] = None;
    }

    /// while creating a new lock, push empty resources
    pub fn add_lock(&mut self, tid: usize, res_count: usize) {
        // add a new lock with resource number
        self.available.push(res_count);
        while tid >= self.thread_num() {
            self.need.push(None);
            self.allocation.push(Vec::new());
        }
        // resize vectors
        let lock_num = self.lock_num();
        for i in 0..self.thread_num() {
            self.allocation[i].resize(lock_num, 0);
        }
        assert!(tid < self.need.len());
    }
    /// do lock/down
    pub fn lock(&mut self, tid: usize, lock_id: usize) {
        self.available[lock_id] -= 1;
        self.allocation[tid][lock_id] += 1;
        self.need[tid] = None;
    }

    /// put 1 resource from allocation to available
    pub fn unlock(&mut self, tid: usize, lock_id: usize) {
        self.available[lock_id] += 1;
        self.allocation[tid][lock_id] -= 1;
    }

    /// return true if current request will cause deadlock
    pub fn is_deadlock(&mut self, tid: usize, lock_id: usize) -> bool {
        self.need[tid] = Some(lock_id);
        let mut not_finished = BTreeSet::new();
        for (tid, alloced) in self.allocation.iter().enumerate() {
            if !alloced.is_empty() {
                not_finished.insert(tid);
            }
        }
        let mut work = self.available.clone();

        while !not_finished.is_empty() {
            let mut finished = BTreeSet::new();
            for tid in &not_finished {
                if let Some(lock_id) = self.need[*tid] {
                    if work[lock_id] == 0 {
                        continue;
                    }
                }
                finished.insert(*tid);
                for (lock_id, resource_num) in self.allocation[*tid].iter().enumerate() {
                    work[lock_id] += resource_num;
                }
            }
            if finished.is_empty() {
                break;
            }
            not_finished = not_finished.difference(&finished).copied().collect();
        }
        !not_finished.is_empty()
    }
}
