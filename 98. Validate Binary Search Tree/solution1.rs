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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::helper(root, None, None);
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, max: Option<i32>, min: Option<i32>) -> bool {
        if let Some(node) = root {
            let cur_val = node.as_ref().borrow().val;
            if let Some(max) = max {
                if max <= cur_val {
                    return false;
                }
            }

            if let Some(min) = min {
                if min >= cur_val {
                    return false;
                }
            }

            let left = node.borrow().left.as_ref().map(Rc::clone);
            let right = node.borrow().right.as_ref().map(Rc::clone);
            return Solution::helper(left, Some(cur_val), min)
                && Solution::helper(right, max, Some(cur_val));
        } else {
            return true;
        }
    }
}
