use std::collections::{HashMap, HashSet};

pub struct Solution {}

struct Ctx<'a> {
    hm: &'a mut HashMap<i32, Vec<i32>>,
    visited: &'a mut HashSet<i32>,
}

fn dfs(cur_node: i32, ctx: &mut Ctx) -> bool {
    if ctx.visited.contains(&cur_node) {
        return false;
    }

    match ctx.hm.get(&cur_node) {
        Some(chls) => {
            if chls.is_empty() {
                return true;
            }
            ctx.visited.insert(cur_node);
            for ch in chls.clone() {
                if !dfs(ch, ctx) {
                    return false;
                }
            }
            ctx.visited.remove(&cur_node);
            ctx.hm.entry(cur_node).or_insert_with(Vec::new).clear();
            true
        }
        None => true,
    }
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut hm = HashMap::<i32, Vec<i32>>::new();
        for i in 0..num_courses {
            hm.insert(i, Vec::<i32>::new());
        }
        for el in prerequisites {
            if let [c1, c2] = &el[..] {
                hm.entry(*c1).or_insert_with(Vec::<i32>::new).push(*c2);
            }
        }
        let mut visited = HashSet::<i32>::new();
        let mut ctx = Ctx {
            hm: &mut hm,
            visited: &mut visited,
        };
        for i in 0..num_courses {
            if !dfs(i, &mut ctx) {
                return false;
            }
        }
        true
    }
}
