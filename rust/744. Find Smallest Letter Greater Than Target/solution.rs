impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l = 0;
        let mut r = letters.len() as i32;
        while l < r {
            let m = (l + r) / 2;
            if letters[m as usize] <= target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l >= letters.len() as i32 {
            l = 0;
        }
        letters[l as usize]
    }
}
