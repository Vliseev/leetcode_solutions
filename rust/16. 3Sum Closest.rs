use std::cmp;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut result = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            let (mut j, mut k) = (i + 1, n - 1);
            while j < k {
                let cur_val = nums[i] + nums[j] + nums[k];
                if i32::abs(target - cur_val) < i32::abs(target - result) {
                    result = cur_val;
                }
                if cur_val == target {
                    return target;
                } else if cur_val < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        result
    }
}
