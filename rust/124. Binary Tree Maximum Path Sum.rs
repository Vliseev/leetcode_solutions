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
use std::cmp::{self, max};

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        static mut ANS: i32 = i32::MIN;
        unsafe { ANS = i32::MIN }

        fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(val) = root.as_ref() {
                let left_max = cmp::max(helper(&val.borrow().left), 0i32);
                let right_max = cmp::max(helper(&val.borrow().right), 0i32);
                let cur_val = val.borrow().val + left_max + right_max;

                unsafe { ANS = max(cur_val, ANS) };
                val.borrow().val + cmp::max(left_max, right_max)
            } else {
                0
            }
        }
        helper(&root);
        unsafe { ANS }
    }
}


#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_max_path_sum() {
        // Example 1: Input: root = [1,2,3], Output: 6
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::max_path_sum(root1), 6);

        // Example 2: Input: root = [-10,9,20,null,null,15,7], Output: 42
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: -10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::max_path_sum(root2), 42);

        // Test case with all negative values
        let root3 = Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::max_path_sum(root3), -1); // The maximum path sum is the single positive value

        // Test case with a balanced tree
        let root4 = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::max_path_sum(root4), 15); // The maximum path sum is 4->5->6
    }
}
