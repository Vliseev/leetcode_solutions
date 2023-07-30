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

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn rec(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, level: &mut i32) -> Option<i32> {
            if let Some(node) = root.as_ref() {
                let left_res = rec(&node.borrow().left, k, level);
                if left_res.is_some() {
                    return left_res;
                }

                *level += 1;
                if *level == k {
                    return Some(node.borrow().val);
                }
                return rec(&node.borrow().right, k, level);
            } else {
                None
            }
        }
        let mut level = 0i32;
        rec(&root, k, &mut level).unwrap()
    }
}// Definition for a binary tree node.
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

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn rec(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, level: &mut i32) -> Option<i32> {
            if let Some(node) = root.as_ref() {
                let left_res = rec(&node.borrow().left, k, level);
                if left_res.is_some() {
                    return left_res;
                }

                *level += 1;
                if *level == k {
                    return Some(node.borrow().val);
                }
                return rec(&node.borrow().right, k, level);
            } else {
                None
            }
        }
        let mut level = 0i32;
        rec(&root, k, &mut level).unwrap()
    }
}

