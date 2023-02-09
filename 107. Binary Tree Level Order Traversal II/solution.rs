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
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut result = VecDeque::<Vec<i32>>::new();
        if let Some(node) = &root {
            queue.push_back(node.clone());
            while !queue.is_empty() {
                let mut cur_res = Vec::<i32>::new();
                let cur_q_size = queue.len();
                for _ in 0..cur_q_size {

                    let cur_node = queue.pop_front().unwrap();
                    let val = cur_node.as_ref().borrow().val;
                    cur_res.push(val);

                    let left = cur_node.as_ref().borrow().left.clone();
                    if let Some(ch) = left {
                        queue.push_back(ch);
                    }

                    let right = cur_node.as_ref().borrow().right.clone();
                    if let Some(ch) = right {
                        queue.push_back(ch);
                    }
                }
                result.push_front(cur_res);
            }
        }
        Vec::<Vec<i32>>::from(result)
    }
}


/*
     let mut tmp_node1: Rc<RefCell<TreeNode>>;
    let tree_node = create_node(3);
    tmp_node1 = tree_node.clone().unwrap();

    tmp_node1.as_ref().borrow_mut().left = create_node(9);
    tmp_node1.as_ref().borrow_mut().right = create_node(20);

    tmp_node1 = tmp_node1.clone().as_ref().borrow().right.clone().unwrap();
    tmp_node1.as_ref().borrow_mut().left = create_node(15);
    tmp_node1.as_ref().borrow_mut().right = create_node(7);
 */
