impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        let n = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[r] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        nums[l]
    }
}
