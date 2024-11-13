fn bisect_left<T: Ord>(nums: &Vec<T>, x: &T, lo: usize) -> usize {
    let mut lo = lo;
    let mut hi = nums.len();
    while lo < hi {
        let m = (lo+hi)/2;
        match nums[m].cmp(x) {
            std::cmp::Ordering::Less => lo = m + 1,
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => hi = m,
        }
    }
    lo
}

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut res = 0;
        for (i, &num) in nums.iter().enumerate() {
            let l = bisect_left(&nums, &(lower - num), i+1);
            let r = bisect_left(&nums, &(upper - num + 1), i+1);
            res += r-l;
        }
        res as i64
    }
}
