/*
Time: O(n)
Space: O(n)

Idea: prefix/sliding count of reachable positions
*/

class Solution {
public:
    bool canReach(string s, int minJump, int maxJump) {
        int n = s.size();

        vector<bool> dp(n, false);
        dp[0] = true;

        int reach = 0;

        for (int i = 1; i < n; i++) {
            // add new left boundary
            if (i >= minJump) {
                reach += dp[i - minJump];
            }

            // remove out-of-range positions
            if (i > maxJump) {
                reach -= dp[i - maxJump - 1];
            }

            // reachable if window has valid position
            if (s[i] == '0' && reach > 0) {
                dp[i] = true;
            }
        }

        return dp[n - 1];
    }
};