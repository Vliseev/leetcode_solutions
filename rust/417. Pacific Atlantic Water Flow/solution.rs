use std::collections::{VecDeque};


impl Solution {
    fn is_pac((x, y): (i32, i32)) -> bool {
        return x == 0 || y == 0;
    }

    fn is_atl((x, y): (i32, i32), (n, m): (i32, i32)) -> bool {
        return x == n - 1 || y == m - 1;
    }

    fn can_flow((x, y): (i32, i32), heights: &Vec<Vec<i32>>) -> bool {
        let (mut pac, mut atl): (bool, bool) = (false, false);
        let (n, m) = (heights.len() as i32, heights[0].len() as i32);
        let mut visited = vec![vec![false; m as usize]; n as usize];

        let mut queue = VecDeque::<(i32, i32)>::new();
        queue.push_back((x, y));
        while let Some((cur_x, cur_y)) = queue.pop_front() {
            pac |= Solution::is_pac((cur_x, cur_y));
            atl |= Solution::is_atl((cur_x, cur_y), (n, m));

            if pac && atl {
                return true;
            }

            visited[x as usize][y as usize] = true;
            for (x_d, y_d) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (new_x, new_y) = (cur_x + x_d, cur_y + y_d);
                if 0 <= new_x && new_x <= n - 1 && 0 <= new_y && new_y <= m - 1 {
                    if !visited[new_x as usize][new_y as usize]
                        && heights[new_x as usize][new_y as usize]
                            <= heights[cur_x as usize][cur_y as usize]
                    {
                        visited[new_x as usize][new_y as usize] = true;
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
        pac && atl
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::<Vec<i32>>::new();
        let (n, m) = (heights.len(), heights[0].len());
        for i in 0..n {
            for j in 0..m {
                let is_can_fl = Solution::can_flow((i as i32, j as i32), &heights);
                if is_can_fl {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }
        result
    }
}
