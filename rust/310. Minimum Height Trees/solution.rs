use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_list = HashMap::<i32, HashSet<i32>>::new();
        for i in 0..n {
            adj_list.insert(i, HashSet::<i32>::new());
        }
        for el in edges {
            if let [c1, c2] = &el[..] {
                adj_list.entry(*c1).and_modify(|el| {
                    el.insert(*c2);
                });
                adj_list.entry(*c2).and_modify(|el| {
                    el.insert(*c1);
                });
            }
        }
        let mut leaves = Vec::<i32>::new();
        for (node, adj) in adj_list.iter() {
            if adj.len() == 1 {
                leaves.push(*node);
            }
        }

        while adj_list.len() > 2 {
            let mut new_leaves = Vec::<i32>::new();
            for l in leaves.iter() {
                let anc = *adj_list.get(l).map(|el| el.iter().next().unwrap()).unwrap();
                adj_list.entry(anc).and_modify(|el| {
                    el.remove(l);
                });
                adj_list.remove(l);

                if let Some(adj_l) = adj_list.get(&anc) {
                    if adj_l.len() == 1 {
                        new_leaves.push(anc);
                    }
                }
            }
            leaves = new_leaves;
        }

        leaves
    }
}

/*
println!(
    "{:#?}",
    Solution::find_min_height_trees(3, vec![vec![0, 1], vec![0, 2]])
);

println!(
    "{:#?}",
    Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
);

println!(
    "{:#?}",
    Solution::find_min_height_trees(6, vec![vec![3,0], vec![3,1], vec![3,2], vec![3,4], vec![5,4]])
);
*/
