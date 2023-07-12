impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0i32;
        let mut col = matrix[0].len() as i32 - 1;
        while row < matrix.len() as i32 && col > -1 {
            match matrix[row as usize][col as usize].cmp(&target) {
                std::cmp::Ordering::Less => row += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => col -= 1,
            }
        }
        false
    }
}

