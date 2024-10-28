impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; m]; n];
        let mut max_square_len = 0;

        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0{
                    dp[i][j] = (matrix[i][j] == '1') as i32
                } else if matrix[i][j] == '1' {
                    dp[i][j] = 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]);
                }
                max_square_len = std::cmp::max(dp[i][j], max_square_len);
            }
        }

        max_square_len * max_square_len
    }
}
