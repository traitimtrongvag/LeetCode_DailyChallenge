class Solution {
public:
    int numberOfPaths(vector<vector<int>>& grid, int k) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<vector<int>>> dp(m, vector<vector<int>>(n, vector<int>(k, 0)));
        int mod = 1e9 + 7;
        
        dp[0][0][grid[0][0] % k] = 1;
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                for (int r = 0; r < k; r++) {
                    if (i > 0) {
                        int newR = (r + grid[i][j]) % k;
                        dp[i][j][newR] = (dp[i][j][newR] + dp[i-1][j][r]) % mod;
                    }
                    if (j > 0) {
                        int newR = (r + grid[i][j]) % k;
                        dp[i][j][newR] = (dp[i][j][newR] + dp[i][j-1][r]) % mod;
                    }
                }
            }
        }
        
        return dp[m-1][n-1][0];
    }
};