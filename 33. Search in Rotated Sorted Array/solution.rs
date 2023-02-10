pub struct Solution {}


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len()-1;

        while l <= r {
            let m = (l+r)/2;
            if nums[m] == target{
                return  m as i32;
            }
            if nums[l] <= nums[m] {
                if target > nums[m] || target < nums[l] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } else {
                if target < nums[m] || target > nums[r] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }
            
        }
        return -1;
    }
}
