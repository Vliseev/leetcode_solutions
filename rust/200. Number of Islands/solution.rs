use std::collections::VecDeque;

struct State<'a> {
    grid: &'a Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}

impl Solution {
    fn dsf((x, y): (i32, i32), state: &mut State) {
        let (n, m) = (state.grid.len() as i32, state.grid[0].len() as i32);

        let mut queue = VecDeque::<(i32, i32)>::new();
        queue.push_back((x, y));
        while let Some((cur_x, cur_y)) = queue.pop_back() {
            for (x_d, y_d) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (new_x, new_y) = (cur_x + x_d, cur_y + y_d);
                if 0 <= new_x && new_x <= n - 1 && 0 <= new_y && new_y <= m - 1 {
                    if !state.visited[new_x as usize][new_y as usize]
                        && state.grid[new_x as usize][new_y as usize] == '1'
                    {
                        state.visited[new_x as usize][new_y as usize] = true;
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut state = State {
            grid: &grid,
            visited: vec![vec![false; m as usize]; n as usize],
        };
        let mut result = 0i32;
        for i in 0..n {
            for j in 0..m {
                if !state.visited[i][j] && state.grid[i][j] == '1' {
                    result += 1;
                    Solution::dsf((i as i32, j as i32), &mut state);
                }
            }
        }

        result
    }
}
