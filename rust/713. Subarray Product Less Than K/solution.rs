impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0i32;
        let (mut fptr, mut cur_prod) = (0, 1);
        for sptr in 0..nums.len() {
            cur_prod *= nums[sptr];
            while cur_prod >= k && fptr <= sptr {
                cur_prod /= nums[fptr];
                fptr += 1;
            }
            if cur_prod < k {
                res += sptr as i32 - fptr as i32 + 1;
            }
        }
        res
    }
}
