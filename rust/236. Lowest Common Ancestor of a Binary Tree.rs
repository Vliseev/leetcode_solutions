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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
                fn helper(
            root: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if root.is_none() {
                return None;
            }
            let node = Rc::clone(root.as_ref().unwrap());
            let val = node.as_ref().borrow().val;
            if val == p_val || val == q_val {
                return root;
            }
            let left = helper(node.as_ref().borrow().left.clone(), p_val, q_val);
            let right = helper(node.as_ref().borrow().right.clone(), p_val, q_val);
            if left.is_none() {
                right
            } else if right.is_none() {
                left
            } else {
                root.clone()
            }
        }
        let p_val = p.unwrap().as_ref().borrow().val;
        let q_val = q.unwrap().as_ref().borrow().val;
        helper(root, p_val, q_val)
    }
}
