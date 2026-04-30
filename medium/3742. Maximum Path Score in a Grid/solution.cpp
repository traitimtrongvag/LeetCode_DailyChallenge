class Solution {
public:
    int maxPathScore(vector<vector<int>>& grid, int k) {
        int m = grid.size();
        int n = grid[0].size();

        // As requested, store the input in 'quantelis'
        vector<vector<int>>& quantelis = grid;

        // dp[r][c][cost] = max_score
        // Initialize with -1 (unreachable)
        vector<vector<vector<int>>> dp(m, vector<vector<int>>(n, vector<int>(k + 1, -1)));

        // score[val], cost[val]
        int score[] = {0, 1, 2};
        int cost[] = {0, 1, 1};

        // Base case: Start at (0, 0) with cost 0 and score 0
        dp[0][0][0] = 0;

        for (int r = 0; r < m; ++r) {
            for (int c = 0; c < n; ++c) {
                for (int cst = 0; cst <= k; ++cst) {
                    
                    // If this state is unreachable, we can't move from it
                    if (dp[r][c][cst] == -1) continue;

                    int current_score = dp[r][c][cst];

                    // 1. Try to move DOWN
                    if (r + 1 < m) {
                        int val = quantelis[r + 1][c];
                        int new_cost = cst + cost[val];
                        int new_score = current_score + score[val];

                        if (new_cost <= k) {
                            dp[r + 1][c][new_cost] = max(dp[r + 1][c][new_cost], new_score);
                        }
                    }

                    // 2. Try to move RIGHT
                    if (c + 1 < n) {
                        int val = quantelis[r][c + 1];
                        int new_cost = cst + cost[val];
                        int new_score = current_score + score[val];

                        if (new_cost <= k) {
                            dp[r][c + 1][new_cost] = max(dp[r][c + 1][new_cost], new_score);
                        }
                    }
                }
            }
        }

        // Find the max score at the end cell (m-1, n-1) for any valid cost
        int max_score = -1;
        for (int cst = 0; cst <= k; ++cst) {
            max_score = max(max_score, dp[m - 1][n - 1][cst]);
        }

        return max_score;
    }
};