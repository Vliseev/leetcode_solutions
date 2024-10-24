// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut levels = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let mut cur_sum = 0;
            let cur_level = queue.len();

            for _ in 0..cur_level {
                if let Some(node) = queue.pop_front() {
                    let node_ref = node.borrow();
                    cur_sum += node_ref.val as i64;

                    if let Some(left) = &node_ref.left {
                        queue.push_back(Rc::clone(left));
                    }

                    if let Some(right) = &node_ref.right {
                        queue.push_back(Rc::clone(right));
                    }
                }
            }
            levels.push(cur_sum);
        }
        levels.sort_unstable_by(|a, b| b.cmp(a));
        if k as usize <= levels.len() {
            return levels[(k - 1) as usize];
        } else {
            return -1;
        }
    }
}
