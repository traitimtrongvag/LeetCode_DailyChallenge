class Solution {
public:
    int zigZagArrays(int n, int l, int r) {
        int MOD = 1e9+7;
        int m = r - l + 1;
        
        // dp0: last element is smaller than previous (going down)
        // dp1: last element is larger than previous (going up)
        vector<int> dp0(m, 1), dp1(m, 1);
        
        for (int len = 1; len < n; len++) {
            vector<int> pref0(m+1, 0), pref1(m+1, 0);
            
            // Build prefix sums for O(1) range queries
            for (int i = 0; i < m; i++) {
                pref0[i+1] = (pref0[i] + dp0[i]) % MOD;
                pref1[i+1] = (pref1[i] + dp1[i]) % MOD;
            }
            
            vector<int> new0(m, 0), new1(m, 0);
            
            for (int i = 0; i < m; i++) {
                // For zig: last was down, now go up -> need next > current
                // So sum all dp1[j] where j > i (larger values)
                new0[i] = (pref1[m] - pref1[i+1] + MOD) % MOD;
                
                // For zag: last was up, now go down -> need next < current
                // So sum all dp0[j] where j < i (smaller values)
                new1[i] = pref0[i];
            }
            
            dp0 = new0;
            dp1 = new1;
        }
        
        long long total = 0;
        for (int i = 0; i < m; i++) {
            total = (total + dp0[i] + dp1[i]) % MOD;
        }
        
        return total;
    }
};