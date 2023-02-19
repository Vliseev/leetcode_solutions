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
        return Solution::helper(&root, None, None);
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max: Option<i32>, min: Option<i32>) -> bool {
        if let Some(node) = root.as_ref() {
            let b = node.borrow();
            if let Some(max) = max {
                if max <= b.val {
                    return false;
                }
            }
            if let Some(min) = min {
                if min >= b.val {
                    return false;
                }
            }
            return Solution::helper(&b.left, Some(b.val), min)
                && Solution::helper(&b.right, max, Some(b.val));
        } else {
            return true;
        }
    }
}
