use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();

        let mut heap: BinaryHeap<(i32, isize, isize)> = BinaryHeap::new();
        let mut visited = vec![vec![false; m]; n];

        for i in 0..n {
            for j in 0..m {
                if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
                    heap.push((-height_map[i][j], i as isize, j as isize));
                    visited[i][j] = true;
                }
            }
        }
        let mut result = 0;
        let directions: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((height, i, j)) = heap.pop() {
            let height = - height;
            for (dx, dy) in directions.iter() {
                let x = (i + dx) as usize;
                let y = (j + dy) as usize;
                if 0 <= x && x < n && 0 <= y && y < m && !visited[x][y] {
                    result += std::cmp::max(0, height - height_map[x][y]);
                    heap.push((
                        -std::cmp::max(height_map[x][y], height),
                        x as isize,
                        y as isize,
                    ));
                    visited[x][y] = true;
                }
            }
        }
        result
    }
}
