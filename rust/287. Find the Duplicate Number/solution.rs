impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = nums.len()-1;
        let mut result = nums[0];

        while left <= right {
            let mid = (left + right) / 2;

            let mut cnt = 0;
            for el in nums.iter() {
                if el <= &(mid as i32) {
                    cnt += 1;
                }
            }
            if cnt > mid as i32 {
                result = mid as i32;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        result
    }
}
