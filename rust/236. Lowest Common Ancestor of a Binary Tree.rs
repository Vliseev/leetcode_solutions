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

impl Solution2 {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(
            root: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                None => None,
                Some(node_rc) => {
                    let node_ref = node_rc.borrow();
                    let current_value = node_ref.val;

                    // If the current node is either p or q, return this node as the LCA candidate.
                    if current_value == p_val || current_value == q_val {
                        return Some(node_rc.clone());
                    }

                    // Recurse on the left and right children.
                    let left_lca = helper(node_ref.left.clone(), p_val, q_val);
                    let right_lca = helper(node_ref.right.clone(), p_val, q_val);

                    // Determine if nodes are found in left or right subtrees.
                    match (left_lca.as_ref(), right_lca.as_ref()) {
                        (Some(_), Some(_)) => Some(node_rc.clone()), // Found LCA in both subtrees.
                        (Some(_), None) => left_lca,
                        (None, Some(_)) => right_lca,
                        (None, None) => None,
                    }
                }
            }
        }
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        helper(root, p_val, q_val)
    }
}
