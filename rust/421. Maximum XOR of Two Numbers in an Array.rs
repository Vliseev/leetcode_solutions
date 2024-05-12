use std::cmp;

struct Solution;

struct TrieNode {
    nodes: [Option<Box<TrieNode>>; 2],
    val: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            nodes: [None, None],
            val: 0,
        }
    }
}

struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    const NUM_BIT: i32 = 31;
    fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new()),
        }
    }

    fn add_number(&mut self, number: i32) {
        let mut cur_node = &mut self.root;
        for bit_id in (0..Self::NUM_BIT).rev() {
            let cur_bit = (number >> bit_id) & 1;
            cur_node = cur_node.nodes[cur_bit as usize].get_or_insert(Box::new(TrieNode::new()));
        }
        cur_node.val = number;
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for num in nums.iter() {
            trie.add_number(*num);
        }

        let mut max_ans = 0;

        for num in nums.iter() {
            let mut node = &trie.root;
            for bit in (0..Trie::NUM_BIT).rev() {
                let cur_bit = (num >> bit) & 1;
                let need_val = 1 - cur_bit;
                node = match &node.nodes[need_val as usize] {
                    Some(n) => n,
                    None => {node.nodes[cur_bit as usize].as_ref().unwrap()},
                };
            }
            max_ans = cmp::max(max_ans, num ^ node.val);
        }

        max_ans
    }
}
fn main() {
    let nums = vec![3, 10, 5, 25, 2];
    assert_eq!(Solution::find_maximum_xor(nums), 28); // The maximum XOR is 5 ^ 27 = 28
}
