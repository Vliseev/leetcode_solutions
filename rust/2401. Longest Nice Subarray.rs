impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut acc = 0u32;
        let mut l = 0usize;
        let mut result = 0i32;
        let n = nums.len();

        for r in 0..n {
            let num = nums[r] as u32;
            while (acc & num) != 0 {
                acc ^= nums[l] as u32;
                l += 1;
            }
            acc |= num;
            result = result.max((r - l + 1) as i32);
        }
        result
    }
}
