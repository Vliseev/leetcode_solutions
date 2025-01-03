impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut running_sum = 0i64;
        let total: i64 = nums.iter().map(|&x| x as i64).sum();
        
        nums.iter()
            .take(nums.len() - 1)
            .filter(|&&x| {
                running_sum += x as i64;
                running_sum >= total - running_sum
            })
            .count() as i32
    }
}
