impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0i64;
        let mut r = nums.len() as i64 - 1;
        while l < r {
            let m = (l+r) / 2;
            if nums[m as usize] == target {
                return true;
            }
            if nums[l as usize] == nums[m as usize] && nums[m as usize] == nums[r as usize] {
                l += 1;
                r -= 1;
                continue;
            }
            if nums[l as usize] <= nums[m as usize] {
                if nums[l as usize] <= target && target < nums[m as usize] {
                    r = m - 1;
                } else {
                    l = m + 1
                }
            } else if nums[m as usize] < target && target <= nums[r as usize] {
                l = m + 1;
            } else {
                r = m - 1
            }
        }
        nums[l as usize] == target
    }
}

