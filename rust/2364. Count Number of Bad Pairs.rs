impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut hm = std::collections::HashMap::new();

        for i in 0..n {
            hm.entry(nums[i] - i as i32).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut tot = n as i64 * (n as i64 - 1) / 2;

        for v in hm.values() {
            tot -= *v as i64 * (*v as i64 - 1) / 2;
        }
        tot
    }
}
