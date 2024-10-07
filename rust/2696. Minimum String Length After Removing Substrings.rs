impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut vec = Vec::new();
        for ch in s.chars() {
            if !vec.is_empty()
                && ((ch == 'B' && *vec.last().unwrap() == 'A')
                    || (ch == 'D' && *vec.last().unwrap() == 'C'))
            {
                vec.pop();
            } else {
                vec.push(ch);
            }
        }
        vec.len() as i32
    }
}
