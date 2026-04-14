class Solution {
public:
    long long minimumTotalDistance(vector<int>& robot, vector<vector<int>>& factory) {
        sort(robot.begin(), robot.end());
        sort(factory.begin(), factory.end());

        int n = robot.size(), m = factory.size();

        // dp[i][j] = min cost using first i robots, first j factories
        vector<vector<long long>> dp(n + 1, vector<long long>(m + 1, LLONG_MAX));
        dp[0][0] = 0;

        for (int j = 1; j <= m; j++) {
            dp[0][j] = 0; // 0 robot -> 0 cost
        }

        for (int j = 1; j <= m; j++) {
            int pos = factory[j - 1][0];
            int cap = factory[j - 1][1];

            for (int i = 1; i <= n; i++) {
                long long cost = 0;

                // try assign k robots to this factory
                for (int k = 1; k <= cap && k <= i; k++) {
                    cost += abs(robot[i - k] - pos);

                    if (dp[i - k][j - 1] != LLONG_MAX) {
                        dp[i][j] = min(dp[i][j], dp[i - k][j - 1] + cost);
                    }
                }

                // skip this factory
                dp[i][j] = min(dp[i][j], dp[i][j - 1]);
            }
        }

        return dp[n][m];
    }
};