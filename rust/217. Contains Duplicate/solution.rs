use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hs = HashSet::new();
        for el in nums.iter() {
            if hs.contains(el) {
                return true;
            }
            hs.insert(el);
        }
        return false;
    }
}
