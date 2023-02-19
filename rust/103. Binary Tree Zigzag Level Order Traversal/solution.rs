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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut result = Vec::<Vec<i32>>::new();
        let mut flag = 1i32;
        if let Some(node) = &root {
            queue.push_back(node.clone());
            while !queue.is_empty() {
                let mut cur_res = VecDeque::<i32>::new();
                let cur_q_size = queue.len();
                for _ in 0..cur_q_size {
                    let cur_node = queue.pop_front().unwrap();
                    let val = cur_node.as_ref().borrow().val;
                    if flag == 1i32 {
                        cur_res.push_back(val);
                    } else {
                        cur_res.push_front(val);
                    }

                    let left = cur_node.as_ref().borrow().left.clone();
                    let right = cur_node.as_ref().borrow().right.clone();

                    if let Some(ch) = left {
                        queue.push_back(ch);
                    }

                    if let Some(ch) = right {
                        queue.push_back(ch);
                    }
                }
                result.push(Vec::<i32>::from(cur_res));
                flag = (flag + 1) % 2;
            }
        }
        result
    }
}


/*
fn create_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        val: val,
        left: None,
        right: None,
    }))
}

fn read_bst(arr: &Vec<Option<i32>>, idx: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if idx >= arr.len() || arr[idx].is_none() {
        return None;
    }

    let node = create_node(arr[idx].unwrap());
    node.as_ref().borrow_mut().left = read_bst(arr, 2 * idx + 1);
    node.as_ref().borrow_mut().right = read_bst(arr, 2 * idx + 2);

    Some(node)
}


let tree_arr = vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)];
let tree = read_bst(&tree_arr, 0);

*/
