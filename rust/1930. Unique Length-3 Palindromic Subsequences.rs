use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut first_occ: HashMap<char, usize> = HashMap::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();
        for (i, ch) in s.chars().enumerate() {
            first_occ.entry(ch).or_insert(i);
            last_occ.insert(ch, i);
        }

        let mut result = 0;

        for (ch, first) in first_occ.iter() {
            let last = last_occ.get(ch).unwrap();
            if first < last {
                let unique_sym = s[first + 1..*last].chars().collect::<HashSet<_>>();
                result += unique_sym.len();
            }
        }

        result as i32
    }
}
