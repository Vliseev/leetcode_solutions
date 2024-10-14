use std::cmp::{max, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut result: i64 = 0;
        let mut heap: BinaryHeap<_> = nums.into_iter().collect();

        for _ in 0..k {
            let el = heap.pop().unwrap();
            result += el as i64;
            let new_val = (el as f64 / 3.0).ceil() as i32;
            heap.push(new_val);
        }

        result
    }
}
