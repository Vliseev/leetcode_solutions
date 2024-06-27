use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let cntr: HashMap<_, _> = t.iter().fold(HashMap::new(), |mut acc, &c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        
        let mut window_chars = HashMap::new();
        let mut left = 0;
        let required = cntr.len();
        let mut formed = 0;
        let mut ans = (0, 0, usize::MAX); // (left, right, length)

        for (right, &sym) in s.iter().enumerate() {
            *window_chars.entry(sym).or_insert(0) += 1;

            if cntr.get(&sym) == window_chars.get(&sym) {
                formed += 1;
            }

            while formed == required {
                if right - left + 1 < ans.2 {
                    ans = (left, right, right - left + 1);
                }
                
                let left_sym = s[left];
                if let Some(count) = window_chars.get_mut(&left_sym) {
                    *count -= 1;
                    if cntr.get(&left_sym).map_or(false, |&req| *count < req) {
                        formed -= 1;
                    }
                }
                left += 1;
            }
        }

        if ans.2 == usize::MAX {
            String::new()
        } else {
            String::from_utf8_lossy(&s[ans.0..=ans.1]).into_owned()
        }
    }
}

#[test]
fn test_min_window_1() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let expected = "BANC".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}

#[test]
fn test_min_window_2() {
    let s = "a".to_string();
    let t = "a".to_string();
    let expected = "a".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}

#[test]
fn test_min_window_3() {
    let s = "a".to_string();
    let t = "aa".to_string();
    let expected = "".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}

#[test]
fn test_min_window_4() {
    let s = "a".to_string();
    let t = "b".to_string();
    let expected = "".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}

#[test]
fn test_min_window_5() {
    let s = "abc".to_string();
    let t = "abc".to_string();
    let expected = "abc".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}

#[test]
fn test_min_window_6() {
    let s = "abc".to_string();
    let t = "ac".to_string();
    let expected = "abc".to_string();
    assert_eq!(Solution::min_window(s, t), expected);
}
