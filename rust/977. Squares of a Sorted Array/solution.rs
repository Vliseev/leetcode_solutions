impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut ptr1, mut ptr2) = (0, nums.len() - 1);

        let mut res = Vec::new();
        while ptr1 <= ptr2 {
            if nums[ptr1].abs() >= nums[ptr2].abs() {
                res.push(nums[ptr1] * nums[ptr1]);
                ptr1 += 1;
            } else {
                res.push(nums[ptr2] * nums[ptr2]);
                ptr2 -= 1;
            }
        }
        res.reverse();
        res
    }
}
