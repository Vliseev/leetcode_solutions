impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut pos = std::collections::HashMap::new();
        let n = mat.len();
        let m = mat[0].len();

        for i in 0..n {
            for j in 0..m {
                pos.insert(mat[i][j], (i, j));
            }
        }

        let mut row_cnt = vec![0; n];
        let mut col_cnt = vec![0; m];
        for i in 0..arr.len() {
            let (x, y) = pos[&arr[i]];
            row_cnt[x] += 1;
            col_cnt[y] += 1;
            if row_cnt[x] == m || col_cnt[y] == n {
                return i as i32;
            }
        }
        -1
    }
}
