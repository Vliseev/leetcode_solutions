use std::{cmp};

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut beg, mut num_zeros, mut res) = (0i32, 0, 0i32);

        for end in 0..nums.len() {
            if nums[end as usize] == 0 {
                num_zeros += 1;
            }
            while num_zeros > k && beg < end as i32 {
                if nums[beg as usize] == 0 {
                    num_zeros -= 1;
                }
                beg += 1;
            }
            if num_zeros <= k {
                res = cmp::max(res, end as i32 - beg + 1);
            }
        }
        res
    }
}
