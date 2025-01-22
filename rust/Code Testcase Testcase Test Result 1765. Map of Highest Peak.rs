use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = is_water.len();
        let m = is_water[0].len();

        let mut height = vec![vec![-1; m]; n];
        let mut queue = VecDeque::new();

        for i in 0..n as isize {
            for j in 0..m as isize {
                if is_water[i as usize][j as usize] == 1 {
                    height[i as usize][j as usize] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        let direction = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((i, j)) = queue.pop_front() {
            let h = height[i as usize][j as usize];
            for (dx, dy) in &direction {
                let ni = i + dx;
                let nj = j + dy;
                if 0 <= ni && ni < n as isize && 0 <= nj && nj < m as isize && height[ni as usize][nj as usize] == -1 {
                    height[ni as usize][nj as usize] = h + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
        height

    }
}
