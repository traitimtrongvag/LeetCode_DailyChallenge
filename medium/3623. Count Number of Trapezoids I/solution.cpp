class Solution {
public:
    static const long long MOD = 1000000007;

    int countTrapezoids(vector<vector<int>>& points) {
        unordered_map<int,int> cntY;
        for (auto &p : points) cntY[p[1]]++;

        long long S = 0, SSq = 0;
        for (auto &kv : cntY) {
            long long c = kv.second;
            if (c >= 2) {
                long long v = c * (c - 1) / 2 % MOD;
                S = (S + v) % MOD;
                SSq = (SSq + v * v) % MOD;
            }
        }

        long long res = (S * S % MOD - SSq + MOD) % MOD;
        res = res * ((MOD + 1) / 2) % MOD; 
        return (int)res;
    }
};