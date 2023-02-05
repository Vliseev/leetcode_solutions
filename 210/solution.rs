use std::collections::{HashMap, HashSet};

struct Ctx<'a> {
    hm: &'a HashMap<i32, Vec<i32>>,
    visited: &'a mut HashSet<i32>,
    loop_detect: &'a mut HashSet<i32>,
    answer: &'a mut Vec<i32>,
}

fn dfs(cur_node: i32, ctx: &mut Ctx) -> bool {
    if ctx.loop_detect.contains(&cur_node) {
        return false;
    }

    if ctx.visited.contains(&cur_node) {
        return true;
    }

    match ctx.hm.get(&cur_node) {
        Some(chls) => {
            ctx.loop_detect.insert(cur_node);
            for ch in chls {
                if !dfs(*ch, ctx) {
                    return false;
                }
            }
            ctx.loop_detect.remove(&cur_node);
            ctx.visited.insert(cur_node);
            ctx.answer.push(cur_node);
            true
        }
        None => true,
    }
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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
        let mut loop_detect = HashSet::<i32>::new();
        let mut result = Vec::<i32>::new();
        let mut ctx = Ctx {
            hm: &hm,
            visited: &mut visited,
            loop_detect: &mut loop_detect,
            answer: &mut result,
        };
        for i in 0..num_courses {
            if !dfs(i, &mut ctx) {
                return Vec::<i32>::new();
            }
        }
        result
    }
}

/*
TEST
println!(
    "{:#?}",
    Solution::find_order(
        4,
        vec![vec![2, 0], vec![1, 0], vec![3, 1], vec![3, 2], vec![1, 3]]
    )
);
*/

