impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let keys = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let mut result = Vec::<String>::new();
        let char_digits = digits.chars().collect::<Vec<_>>();
        let mut cur = Vec::<char>::new();
        fn rec_foo(
            sym_id: usize,
            digits: &Vec<char>,
            cur: &mut Vec<char>,
            res: &mut Vec<String>,
            keys: &Vec<Vec<char>>,
        ) {
            if sym_id == digits.len() {
                res.push(cur.iter().collect());
                return;
            }
            let ch_id = digits[sym_id] as usize - '0' as usize - 2;
            for char in &keys[ch_id] {
                cur.push(*char);
                rec_foo(sym_id + 1, digits, cur, res, keys);
                cur.pop();
            }
        }
        if !digits.is_empty() {
            rec_foo(0, &char_digits, &mut cur, &mut result, &keys);
        }
        result
    }
}
