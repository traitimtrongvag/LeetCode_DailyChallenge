impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let mod_val = 1_000_000_007;
        
        let mut dp = vec![vec![vec![0; k]; n]; m];
        dp[0][0][(grid[0][0] as usize) % k] = 1;
        
        for i in 0..m {
            for j in 0..n {
                for r in 0..k {
                    if i > 0 {
                        let new_r = (r + grid[i][j] as usize) % k;
                        dp[i][j][new_r] = (dp[i][j][new_r] + dp[i-1][j][r]) % mod_val;
                    }
                    if j > 0 {
                        let new_r = (r + grid[i][j] as usize) % k;
                        dp[i][j][new_r] = (dp[i][j][new_r] + dp[i][j-1][r]) % mod_val;
                    }
                }
            }
        }
        
        dp[m-1][n-1][0]
    }
}