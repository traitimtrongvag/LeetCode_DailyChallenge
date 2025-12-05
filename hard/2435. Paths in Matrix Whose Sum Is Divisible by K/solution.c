int numberOfPaths(int** grid, int gridSize, int* gridColSize, int k) {
    int m = gridSize, n = gridColSize[0];
    int mod = 1000000007;
    
    int*** dp = (int***)malloc(m * sizeof(int**));
    for (int i = 0; i < m; i++) {
        dp[i] = (int**)malloc(n * sizeof(int*));
        for (int j = 0; j < n; j++) {
            dp[i][j] = (int*)calloc(k, sizeof(int));
        }
    }
    
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
    
    int result = dp[m-1][n-1][0];
    
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            free(dp[i][j]);
        }
        free(dp[i]);
    }
    free(dp);
    
    return result;
}