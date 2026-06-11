class Solution {
public:
    int assignEdgeWeights(vector<vector<int>>& e) {
        const int MOD = 1e9 + 7;
        int n = e.size() + 1;

        vector<vector<int>> g(n + 1);
        for (auto &x : e) {
            g[x[0]].push_back(x[1]);
            g[x[1]].push_back(x[0]);
        }

        function<int(int,int)> dfs = [&](int u, int p) {
            int mx = 0;
            for (int v : g[u]) {
                if (v == p) continue;
                mx = max(mx, dfs(v, u) + 1);
            }
            return mx;
        };

        int d = dfs(1, 0);
        long long ans = 1, a = 2;
        int b = d - 1;

        while (b) {
            if (b & 1) ans = ans * a % MOD;
            a = a * a % MOD;
            b >>= 1;
        }

        return ans;
    }
};