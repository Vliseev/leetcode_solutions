impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();

        let (n, m) = (words1.len(), words2.len());

        let (mut i, mut j) = (0, 0);

        while i < words1.len() && i < words2.len() && words1[i] == words2[i] {
            i += 1;
        }
        while j < words1.len() && j < words2.len() && words1[n - j - 1] == words2[m - j - 1] {
            j += 1;
        }

        i + j >= n || i + j >= m
    }
}
