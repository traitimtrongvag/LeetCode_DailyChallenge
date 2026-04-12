class Solution {
public:
    int minimumDistance(string word) {
        int n = word.size();
        
        auto dist = [](int a, int b) -> int {
            if (a == 26) return 0;
            return abs(a/6 - b/6) + abs(a%6 - b%6);
        };
        
        // dp[j] = min cost when one finger at word[i], other finger at position j
        // 26 means finger not placed yet
        vector<int> dp(27, INT_MAX);
        dp[26] = 0;
        
        for (int i = 0; i < n - 1; i++) {
            int cur = word[i] - 'A';
            int nxt = word[i+1] - 'A';
            vector<int> ndp(27, INT_MAX);
            
            for (int j = 0; j <= 26; j++) {
                if (dp[j] == INT_MAX) continue;
                // move finger at cur to nxt, other stays at j
                if (ndp[j] > dp[j] + dist(cur, nxt))
                    ndp[j] = dp[j] + dist(cur, nxt);
                // move finger at j to nxt, other stays at cur
                if (ndp[cur] > dp[j] + dist(j, nxt))
                    ndp[cur] = dp[j] + dist(j, nxt);
            }
            dp = ndp;
        }
        
        return *min_element(dp.begin(), dp.end());
    }
};