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
use std::collections::HashMap;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut height_dict: HashMap<*const TreeNode, i32> = HashMap::new();

        fn find_height(
            node: &Option<Rc<RefCell<TreeNode>>>,
            height_dict: &mut HashMap<*const TreeNode, i32>,
        ) -> i32 {
            if let Some(rrr) = node.as_ref() {
                let node_ref = rrr.borrow();
                let left = find_height(&node_ref.left.clone(), height_dict);
                let right = find_height(&node_ref.right.clone(), height_dict);
                height_dict.insert(rrr.as_ptr(), std::cmp::max(left, right) + 1);
                height_dict[&(&*node_ref as *const TreeNode)]
            } else {
                0
            }
        };

        find_height(&root, &mut height_dict);

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            cur_h: i32,
            max_so_far: i32,
            height_dict: &HashMap<*const TreeNode, i32>,
            query_val: &mut HashMap<i32, i32>,
        ) {
            if let Some(n) = node {
                let node_ref = n.borrow();
                query_val.insert(node_ref.val, max_so_far);

                let left_heigh: i32 = if let Some(nn) = &node_ref.left {
                    height_dict[&(&*nn.borrow() as *const TreeNode)]
                } else {
                    0
                };

                let right_height: i32 = if let Some(nn) = &node_ref.right {
                    height_dict[&(&*nn.borrow() as *const TreeNode)]
                } else {
                    0
                };

                dfs(
                    &node_ref.left.clone(),
                    cur_h + 1,
                    std::cmp::max(max_so_far, cur_h + right_height),
                    height_dict,
                    query_val,
                );
                dfs(
                    &node_ref.right.clone(),
                    cur_h + 1,
                    std::cmp::max(max_so_far, cur_h + left_heigh),
                    height_dict,
                    query_val,
                );
            }
        }

        let mut query_val: HashMap<i32, i32> = HashMap::new();
        dfs(&root, 0, 0, &height_dict, &mut query_val);

        queries.into_iter().map(|q| query_val[&q]).collect()
    }
}
