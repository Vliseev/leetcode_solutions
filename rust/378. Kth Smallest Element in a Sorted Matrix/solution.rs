fn find_less_number(matrix: &Vec<Vec<i32>>, target: i32) -> i32 {
    let mut cnt = 0;
    let n = matrix.len();
    let (mut i, mut j) = (matrix.len() as i32 - 1, 0usize);
    while i >= 0 && j < n {
        if matrix[i as usize][j] <= target {
            cnt += i + 1;
            j += 1;
        } else {
            i -= 1;
        }
    }
    cnt as i32
}


impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut left = matrix[0][0] - 1;
        let mut right = matrix[n - 1][n - 1];
        while left + 1 < right {
            let mid = (left + right) / 2;
            let cnt = find_less_number(&matrix, mid);
            if cnt < k {
                left = mid;
            } else {
                right = mid;
            }
        }
        right
    }
}

