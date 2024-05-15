use std::{cmp, collections::HashSet};

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels: HashSet<u8> = "aeiou".bytes().collect();

        let s = s.as_bytes();
        let k = k as usize;

        let mut max_cnt = s
            .iter()
            .take(k as usize)
            .filter(|&c| vowels.contains(c))
            .count() as i32;
        let mut cur_cnt = max_cnt;

        for i in k as usize..s.len() {
            cur_cnt = cur_cnt - vowels.contains(&s[i - k]) as i32 + vowels.contains(&s[i]) as i32;
            max_cnt = cmp::max(max_cnt, cur_cnt);
        }

        max_cnt
    }
}
