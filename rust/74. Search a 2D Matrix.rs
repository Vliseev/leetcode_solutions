impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut top = 0i32;
        let mut bot = matrix.len() as i32 - 1;
        while top < bot {
            let m = (top + bot) / 2;
            if target < matrix[m as usize][0] {
                bot = m - 1;
            } else if target > *matrix[m as usize].last().unwrap() {
                top = m + 1;
            } else {
                top = m;
                bot = m;
                break;
            }
        }
        if top > bot {
            return false;
        }
        let mut l = 0i32;
        let mut r = matrix[0].len() as i32 - 1;
        while l <= r {
            let m = (l+r)/2;

            match target.cmp(&matrix[top as usize][m as usize]) {
                std::cmp::Ordering::Less => r = m - 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => l = m + 1,
            }
        }

        false
    }
}

