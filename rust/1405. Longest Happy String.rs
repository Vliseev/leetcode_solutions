use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        for (cnt, ch) in [(a, 'a' as u8), (b, 'b' as u8), (c, 'c' as u8)] {
            if cnt > 0 {
                heap.push((cnt, ch));
            }
        }
        let mut result = Vec::new();
        while let Some((cnt, ch)) = heap.pop() {
            let len_res = result.len();
            if len_res >= 2
                && result[len_res - 1] == result[len_res - 2]
                && result[len_res - 1] == ch
            {
                if let Some((mut cnt1, ch1)) = heap.pop() {
                    result.push(ch1);
                    cnt1 -= 1;
                    if cnt1 > 0 {
                        heap.push((cnt1, ch1));
                    }
                    heap.push((cnt, ch));
                } else {
                    break;
                }
            } else {
                result.push(ch);
                if cnt > 1 {
                    heap.push((cnt - 1, ch));
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}
