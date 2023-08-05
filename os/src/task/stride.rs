use core::cmp::{Ordering, Reverse};

use alloc::collections::BinaryHeap;

/// a large constant as divide num
pub const BIG_STRIDE: usize = core::usize::MAX;

/// a usize struct represented stride, considered overflow
#[derive(Debug)]
pub struct Stride(pub usize);

impl Ord for Stride {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 > other.0 {
            if self.0 - other.0 <= BIG_STRIDE / 2 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        } else if other.0 - self.0 <= BIG_STRIDE / 2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Stride {}

impl From<usize> for Stride {
    fn from(other: usize) -> Self {
        Stride(other)
    }
}

impl Stride {
    /// add stride num
    pub fn add(&mut self, other: usize) {
        self.0 += other;
    }
}

/// stride struct test.
/// assume that stride_max - stride_min <= big_stride/2,
/// when stride as u8 and big_stride = 255, we have
/// 125 < 255 is false
/// 129 < 255 is true
#[allow(unused)]
pub fn stride_test() {
    assert!(Stride(BIG_STRIDE / 2 - 2) > Stride(BIG_STRIDE));
    assert!(Stride(BIG_STRIDE / 2 + 2) < Stride(BIG_STRIDE));
    println!("stride test passed!");
}

/// stride b-heap test.
#[allow(unused)]
pub fn stride_heap_test() {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Stride(6)));
    heap.push(Reverse(Stride(0)));
    heap.push(Reverse(Stride(1)));
    assert_eq!(0, heap.pop().unwrap().0 .0);
    assert_eq!(1, heap.pop().unwrap().0 .0);
    assert_eq!(6, heap.pop().unwrap().0 .0);
    println!("stride heap test passed!");
}
