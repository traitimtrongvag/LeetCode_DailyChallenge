class Solution {
public:
    int MOD = 1e9 + 7;
    const int LOG = 20;
    
    vector<vector<int>> adj;
    vector<vector<int>> up;
    vector<int> depth;
    
    long long modpower(long long n) {
        if (n == 0) return 1;
        long long res = modpower(n / 2);
        res = (res * res) % MOD;
        if (n & 1) res = (res * 2) % MOD;
        return res;
    }
    
    void dfs(int u, int p) {
        up[u][0] = p;
        for (int v : adj[u]) {
            if (v != p) {
                depth[v] = depth[u] + 1;
                dfs(v, u);
            }
        }
    }
    
    int lca(int u, int v) {
        if (depth[u] < depth[v]) swap(u, v);
        
        int diff = depth[u] - depth[v];
        for (int i = 0; i < LOG; i++) {
            if (diff >> i & 1) u = up[u][i];
        }
        
        if (u == v) return u;
        
        for (int i = LOG - 1; i >= 0; i--) {
            if (up[u][i] != up[v][i]) {
                u = up[u][i];
                v = up[v][i];
            }
        }
        return up[u][0];
    }
    
    vector<int> assignEdgeWeights(vector<vector<int>>& edges, vector<vector<int>>& queries) {
        int n = edges.size() + 1;
        
        adj.resize(n);
        up.assign(n, vector<int>(LOG, -1));
        depth.assign(n, 0);
        
        for (auto& e : edges) {
            int u = e[0] - 1, v = e[1] - 1;
            adj[u].push_back(v);
            adj[v].push_back(u);
        }
        
        dfs(0, 0);
        
        for (int j = 1; j < LOG; j++) {
            for (int i = 0; i < n; i++) {
                if (up[i][j - 1] != -1)
                    up[i][j] = up[up[i][j - 1]][j - 1];
            }
        }
        
        vector<int> ans;
        for (auto& q : queries) {
            int u = q[0] - 1, v = q[1] - 1;
            
            if (u == v) {
                ans.push_back(0);
                continue;
            }
            
            int p = lca(u, v);
            int edgesCount = depth[u] + depth[v] - 2 * depth[p];
            ans.push_back(modpower(edgesCount - 1));
        }
        
        return ans;
    }
};