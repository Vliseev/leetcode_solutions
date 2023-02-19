use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut hm = HashMap::from([(0, 1)]);
        let mut cum_sum = 0i32;
        let mut result = 0i32;

        for el in nums.iter() {
            cum_sum += el;
            result += *hm.get(&(cum_sum - k)).get_or_insert(&0);
            *hm.entry(cum_sum).or_insert(0) += 1;
        }
        result
    }
}
