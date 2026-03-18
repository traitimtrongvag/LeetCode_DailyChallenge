impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut ans = 0;
        let mut prefix = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                prefix[i][j] = grid[i][j];

                if i > 0 {
                    prefix[i][j] += prefix[i - 1][j];
                }
                if j > 0 {
                    prefix[i][j] += prefix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    prefix[i][j] -= prefix[i - 1][j - 1];
                }

                // submatrix from (0,0) to (i,j)
                if prefix[i][j] <= k {
                    ans += 1;
                }
            }
        }

        ans
    }
}