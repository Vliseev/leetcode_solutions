use std::collections::HashMap;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let mut hm = nums.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });
        let mut cur: Vec<i32> = vec![];
        fn rec(
            cur: &mut Vec<i32>,
            hm: &mut HashMap<i32, i32>,
            result: &mut Vec<Vec<i32>>,
            len: usize,
        ) {
            if cur.len() == len {
                result.push(cur.clone());
            }

            let keys: Vec<i32> = hm
                .keys()
                .cloned()
                .filter(|k| *hm.get(k).unwrap() > 0)
                .collect();
            for k in &keys {
                if *hm.get(k).unwrap() > 0 {
                    *hm.get_mut(k).unwrap() -= 1;
                    cur.push(*k);
                    rec(cur, hm, result, len);
                    cur.pop();
                    *hm.get_mut(k).unwrap() += 1;
                }
            }
        }
        rec(&mut cur, &mut hm, &mut result, nums.len());

        result
    }
}

