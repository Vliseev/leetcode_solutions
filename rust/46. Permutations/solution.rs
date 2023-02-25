use std::collections::VecDeque;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(nums: &mut VecDeque<i32>) -> Vec<Vec<i32>> {
            let mut result = Vec::<Vec<i32>>::new();
            if nums.len() == 1 {
                let n = nums[0];
                result.push(vec![n]);
                return result;
            }
            let size = nums.len();
            for _ in 0..size {
                let val = nums.pop_front().unwrap();
                let mut perms = rec(nums);
                for p in &mut perms {
                    p.push(val);
                }
                result.extend(perms);
                nums.push_back(val);
            }
            result
        }
        let mut nums_vd = VecDeque::<i32>::from(nums);
        rec(&mut nums_vd)
    }
}

