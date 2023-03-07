use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let (mut hs1, mut hs2) = (HashMap::<char, usize>::new(), HashMap::<char, usize>::new());
        let (n, m) = (s1.len(), s2.len());

        let s1ch = s1.chars().collect::<Vec<_>>();
        let s2ch = s2.chars().collect::<Vec<_>>();

        for i in 0..n {
            *hs1.entry(s1ch[i]).or_insert(0) += 1;
            *hs2.entry(s2ch[i]).or_insert(0) += 1;
        }

        let exp_match = hs1.len();
        let mut num_match = 0;
        for k in hs1.keys() {
            if let Some(val) = hs2.get(k) {
                num_match += (*val == hs1[k]) as usize;
            }
        }

        let mut l = 0usize;
        for r in n..m {
            if num_match == exp_match {
                return true;
            }

            let new_sym = s2ch[r];
            let rem_sym = s2ch[l];

            *hs2.entry(new_sym).or_insert(0) += 1;
            if let Some(val1) = hs1.get(&new_sym) {
                let val2 = hs2.get(&new_sym).unwrap();
                if val1 == val2 {
                    num_match += 1;
                } else if *val2 == *val1 + 1 {
                    num_match -= 1;
                }
            }
            *hs2.get_mut(&rem_sym).unwrap() -= 1;
            if let Some(val1) = hs1.get(&rem_sym) {
                let val2 = hs2.get(&rem_sym).unwrap();
                if val1 == val2 {
                    num_match += 1;
                } else if *val2 + 1 == *val1 {
                    num_match -= 1;
                }
            }
            l += 1
        }

        return num_match == exp_match;
    }
}
