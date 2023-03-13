impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut l, mut r) = (0i32, nums.len() as i32-1);
        while l <= r {
            let m = (r + l) / 2;
            if nums[m as usize] == target {
                return m as i32;
            } else if target < nums[m as usize] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        -1
    }
}
