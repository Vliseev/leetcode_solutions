use std::collections::{HashMap, HashSet};

fn find_heig(node: i32, adj_list: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> i32 {
    let mut heigh = 0i32;
    visited.insert(node);
    if let Some(children) = adj_list.get(&node) {
        for ch in children {
            if !visited.contains(ch) {
                let cur_h = find_heig(*ch, adj_list, visited) + 1;
                heigh = std::cmp::max(cur_h, heigh);
            }
        }
    }
    heigh
}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj_list = HashMap::<i32, Vec<i32>>::new();
        let mut vertex_heigh = HashMap::<i32, i32>::new();
        for i in 0..n {
            adj_list.insert(i, Vec::<i32>::new());
        }
        for el in edges {
            if let [c1, c2] = &el[..] {
                adj_list.entry(*c1).and_modify(|v| v.push(*c2));
                adj_list.entry(*c2).and_modify(|v| v.push(*c1));
            }
        }
        for node in 0..n {
            let mut visited = HashSet::<i32>::new();
            let heigh = find_heig(node, &adj_list, &mut visited);
            vertex_heigh.insert(node, heigh);
        }

        let heighs: Vec<(i32, i32)> = vertex_heigh.into_iter().collect();
        let min_h = heighs.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
        heighs
            .iter()
            .filter(|el| el.1.eq(&min_h))
            .map(|x| x.0)
            .collect()
    }
}
