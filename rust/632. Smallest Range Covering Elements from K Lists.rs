use std::cmp::{self, Reverse};
use std::collections::BinaryHeap;

struct Solution;


impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut hp = BinaryHeap::new();
        let mut max_el = i32::MIN;
        for i in 0..nums.len() {
            let f_el = nums[i][0];
            hp.push(Reverse((f_el, 0usize, i)));
            max_el = cmp::max(max_el, f_el);
        }

        let (mut left_bound, mut right_bound) = (i32::MIN / 2, i32::MAX / 2);
        while let Some(Reverse((min_el, pos, idx))) = hp.pop() {
            if max_el - min_el < right_bound - left_bound {
                (left_bound, right_bound) = (min_el, max_el);
            } else if max_el - min_el == right_bound - left_bound
                && max_el < right_bound
                && min_el > left_bound
            {
                (left_bound, right_bound) = (min_el, max_el);
            }
            if pos + 1 == nums[idx].len() {
                break;
            }
            let next_el = nums[idx][pos + 1];
            max_el = cmp::max(max_el, next_el);
            hp.push(Reverse((next_el, pos + 1, idx)));
        }
        vec![left_bound, right_bound]
    }
}

fn main() {
    let lists = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];
    let result = Solution::smallest_range(lists);
    println!("The smallest range is {:?}", result);
}
