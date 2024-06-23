use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut result = 0;
        let bytes = s.as_bytes();
        let length = bytes.len();

        let mut previous_positions = vec![-1isize; length];
        let mut next_positions = vec![length as isize; length];

        let mut position_map = HashMap::new();

        // Fill the previous_positions vector
        for (i, &ch) in bytes.iter().enumerate() {
            if let Some(&pos) = position_map.get(&ch) {
                previous_positions[i] = pos;
            }
            position_map.insert(ch, i as isize);
        }

        position_map.clear();

        // Fill the next_positions vector
        for (i, &ch) in bytes.iter().enumerate().rev() {
            if let Some(&pos) = position_map.get(&ch) {
                next_positions[i] = pos;
            }
            position_map.insert(ch, i as isize);
        }

        // Calculate the result
        for i in 0..length {
            let left_count = i as isize - previous_positions[i];
            let right_count = next_positions[i] - i as isize;
            result += (left_count * right_count) as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_letter_string() {
        assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
        assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
        assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
    }
}
