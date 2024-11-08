impl Solution {
    pub fn get_maximum_xor(mut nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut cum_xor = nums.iter().fold(0, |acc, &n| acc ^ n);
        let max_k = (1 << maximum_bit) - 1;

        let mut result = Vec::with_capacity(nums.len());
        for _ in 0..nums.len() {
            let k = max_k ^ cum_xor;
            result.push(k);
            cum_xor ^= nums.pop().unwrap();
        }
        result
    }
}
