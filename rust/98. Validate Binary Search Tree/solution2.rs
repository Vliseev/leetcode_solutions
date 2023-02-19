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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Solution::helper(root, None, None);
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, max: Option<i32>, min: Option<i32>) -> bool {
        if root.is_none() {
            return true;
        }
        let node = Rc::clone(root.as_ref().unwrap());
        let cur_val = node.as_ref().borrow().val;
        if max.is_some() && (max.unwrap() <= cur_val) {
            return false;
        }
        if min.is_some() && (min.unwrap() >= cur_val) {
            return false;
        }
        let left = node.as_ref().borrow().left.clone();
        let right = node.as_ref().borrow().right.clone();
        return Solution::helper(left, Some(cur_val), min)
            && Solution::helper(right, max, Some(cur_val));
    }
}
