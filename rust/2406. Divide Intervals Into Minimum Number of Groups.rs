use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut heap = BinaryHeap::new();
        let mut max_groups = 0;

        for interval in intervals.iter() {
            while let Some(Reverse(top)) = heap.peek() {
                if top < &interval[0] {
                    heap.pop();
                } else {
                    break;
                }
            }
            heap.push(Reverse(interval[1]));
            max_groups = std::cmp::max(max_groups, heap.len());
        }

        max_groups as i32
    }
}
