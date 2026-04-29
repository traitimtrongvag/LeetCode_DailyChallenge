class Solution {
public:
    long long maximumScore(vector<vector<int>>& grid) {
        int n = grid.size();
        long long preSum[2][n + 1];
        preSum[0][0] = preSum[1][0] = 0;
        for (int i = 0; i < n; i++)
            preSum[0][i + 1] = preSum[0][i] + grid[i][0];

        long long dp[2][n + 1][2];
        memset(dp, 0, sizeof(dp));
        int prev = 0, curr = 1;
        for (int col = 0; col < n - 1; col++) {
            for (int i = 0; i < n; i++)
                preSum[curr][i + 1] = preSum[curr][i] + grid[i][col + 1];
            
            long long preMax = dp[prev][0][1];
            for (int nextBlack = 1; nextBlack <= n; nextBlack++) {
                dp[curr][nextBlack][0] = dp[curr][nextBlack][1]
                    = max(dp[prev][nextBlack][0], preMax + preSum[prev][nextBlack]);
                preMax = max(preMax, dp[prev][nextBlack][1] - preSum[prev][nextBlack]);
            }

            long long sufMax = dp[prev][n][0] + preSum[curr][n];
            for (int nextBlack = n - 1; nextBlack > 0; nextBlack--) {
                dp[curr][nextBlack][0] = max(dp[curr][nextBlack][0],
                                                sufMax - preSum[curr][nextBlack]);
                sufMax = max(sufMax, dp[prev][nextBlack][0] + preSum[curr][nextBlack]);
            }

            dp[curr][0][0] = sufMax;
            dp[curr][0][1] = max(dp[prev][0][0], dp[prev][n][0]);
            swap(prev, curr);
        }

        return *max_element((long long*)dp[prev], (long long*)dp[prev] + (n + 1) * 2);
    }
};