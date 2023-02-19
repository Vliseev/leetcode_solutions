impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums: Vec<_> = nums.iter().enumerate().collect();
        sorted_nums.sort_by(|a, b| a.1.cmp(b.1));
        let (mut idx1, mut idx2): (usize, usize) = (0, sorted_nums.len() - 1);
        while idx1 < idx2 {
            let cur_val: i32 = sorted_nums[idx1].1 + sorted_nums[idx2].1;
            if cur_val < target {
                idx1 += 1
            } else if cur_val > target {
                idx2 -= 1
            } else {
                return vec![sorted_nums[idx1].0 as i32, sorted_nums[idx2].0 as i32];
            }
        }
        vec![sorted_nums[idx1].0 as i32, sorted_nums[idx2].0 as i32]
    }
}
