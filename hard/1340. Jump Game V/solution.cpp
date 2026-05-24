/*
Time: O(n * d)
Space: O(n)

Idea: DFS + memo, jump left/right while values decrease
*/

class Solution {
public:
    vector<int> dp;

    int dfs(vector<int>& arr, int d, int i) {
        if (dp[i] != -1) {
            return dp[i];
        }

        int best = 1;

        // go left
        for (int j = i - 1; j >= max(0, i - d); j--) {
            if (arr[j] >= arr[i]) {
                break;
            }

            best = max(best, 1 + dfs(arr, d, j));
        }

        // go right
        for (int j = i + 1; j <= min((int)arr.size() - 1, i + d); j++) {
            if (arr[j] >= arr[i]) {
                break;
            }

            best = max(best, 1 + dfs(arr, d, j));
        }

        return dp[i] = best;
    }

    int maxJumps(vector<int>& arr, int d) {
        int n = arr.size();

        dp.assign(n, -1);

        int ans = 1;

        for (int i = 0; i < n; i++) {
            ans = max(ans, dfs(arr, d, i));
        }

        return ans;
    }
};