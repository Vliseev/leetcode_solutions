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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::<(Rc<RefCell<TreeNode>>, i32)>::new();
        let mut result = Vec::<Vec<i32>>::new();
        let mut max_deph = -1;
        if let Some(node) = &root {
            queue.push_back((node.clone(), 0i32));
            while let Some((cur_node, cur_level)) = queue.pop_front() {
                if cur_level > max_deph {
                    max_deph +=1;
                    result.push(Vec::<i32>::new());
                }

                result[cur_level as usize].push(cur_node.as_ref().borrow().val);
                if let Some(ch) = cur_node.as_ref().borrow().left.clone() {
                    queue.push_back((ch, cur_level+1));
                }

                if let Some(ch) = cur_node.as_ref().borrow().right.clone() {
                    queue.push_back((ch, cur_level+1));
                }
            }
        }
        result
    }
}


/*
{
    let mut tmp_node1: Rc<RefCell<TreeNode>>;
    let tree_node = create_node(3);
    tmp_node1 = tree_node.clone().unwrap();
    
    tmp_node1.as_ref().borrow_mut().left = create_node(9);
    tmp_node1.as_ref().borrow_mut().right = create_node(20);
    
    tmp_node1 = tmp_node1.clone().as_ref().borrow().right.clone().unwrap();
    tmp_node1.as_ref().borrow_mut().left = create_node(15);
    tmp_node1.as_ref().borrow_mut().right = create_node(7);
}
*/
