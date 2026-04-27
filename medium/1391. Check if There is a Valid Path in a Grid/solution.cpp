class Solution {
public:
    bool hasValidPath(vector<vector<int>>& g) {
        int m = g.size(), n = g[0].size();

        // allowed directions for each street type
        vector<vector<int>> d = {
            {}, {0,1}, {2,3}, {0,3}, {1,3}, {0,2}, {1,2}
        };

        // {dr, dc, outDir, inDir}
        int mv[4][4] = {
            {0, -1, 0, 1},  // left
            {0, 1, 1, 0},   // right
            {-1, 0, 2, 3},  // up
            {1, 0, 3, 2}    // down
        };

        vector<vector<bool>> vis(m, vector<bool>(n, false));
        queue<pair<int,int>> q;

        q.push({0,0});
        vis[0][0] = true;

        while (!q.empty()) {
            auto [r, c] = q.front(); 
            q.pop();

            if (r == m - 1 && c == n - 1) return true;

            for (auto &t : mv) {
                int nr = r + t[0], nc = c + t[1];
                int out = t[2], in = t[3];

                if (nr < 0 || nr >= m || nc < 0 || nc >= n) continue;
                if (vis[nr][nc]) continue;

                // check connection both sides
                if (find(d[g[r][c]].begin(), d[g[r][c]].end(), out) != d[g[r][c]].end() &&
                    find(d[g[nr][nc]].begin(), d[g[nr][nc]].end(), in) != d[g[nr][nc]].end()) {

                    vis[nr][nc] = true;
                    q.push({nr, nc});
                }
            }
        }

        return false;
    }
};