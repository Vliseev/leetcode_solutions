impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort();
        let mut ws = std::collections::HashSet::new();
        let mut ans = "".to_string();
        for w in &words {
            if w.len() == 1 {
                ws.insert(w);
                if ans.is_empty() {
                    ans = w.to_string();
                }
            } else {
                if ws.contains(&w[..w.len() - 1].to_string()) {
                    ws.insert(w);
                    if ans.len() < w.len() {
                        ans = w.to_string();
                    }
                }
            }
        }
        ans
    }
}
