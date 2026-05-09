/*
Time: O(m * n)
Space: O(m * n)

Idea: process each layer separately, rotate elements by k
*/

class Solution {
public:
    vector<vector<int>> rotateGrid(vector<vector<int>>& grid, int k) {
        int m = grid.size();
        int n = grid[0].size();

        vector<vector<int>> res = grid;

        int layers = min(m, n) / 2;

        for (int layer = 0; layer < layers; layer++) {
            vector<int> vals;

            int top = layer;
            int bottom = m - layer - 1;
            int left = layer;
            int right = n - layer - 1;

            // top
            for (int j = left; j <= right; j++) {
                vals.push_back(grid[top][j]);
            }

            // right
            for (int i = top + 1; i < bottom; i++) {
                vals.push_back(grid[i][right]);
            }

            // bottom
            for (int j = right; j >= left; j--) {
                vals.push_back(grid[bottom][j]);
            }

            // left
            for (int i = bottom - 1; i > top; i--) {
                vals.push_back(grid[i][left]);
            }

            int len = vals.size();
            int shift = k % len;

            vector<int> rot(len);

            // rotate left
            for (int i = 0; i < len; i++) {
                rot[i] = vals[(i + shift) % len];
            }

            int idx = 0;

            // top
            for (int j = left; j <= right; j++) {
                res[top][j] = rot[idx++];
            }

            // right
            for (int i = top + 1; i < bottom; i++) {
                res[i][right] = rot[idx++];
            }

            // bottom
            for (int j = right; j >= left; j--) {
                res[bottom][j] = rot[idx++];
            }

            // left
            for (int i = bottom - 1; i > top; i--) {
                res[i][left] = rot[idx++];
            }
        }

        return res;
    }
};