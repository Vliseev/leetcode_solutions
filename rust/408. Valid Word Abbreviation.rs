impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let (word, abbr) = (word.as_bytes(), abbr.as_bytes());
        let (mut i, mut j) = (0, 0);

        while i < word.len() && j < abbr.len() {
            match abbr[j] {
                b'0' => return false,
                b'1'..=b'9' => {
                    let mut num = 0;
                    while j < abbr.len() && abbr[j].is_ascii_digit() {
                        num = num * 10 + (abbr[j] - b'0') as usize;
                        j += 1;
                    }
                    i += num;
                }
                _ => {
                    if word.get(i) != abbr.get(j) {
                        return false;
                    }
                    i += 1;
                    j += 1;
                }
            }
        }

        i == word.len() && j == abbr.len()
    }
}
