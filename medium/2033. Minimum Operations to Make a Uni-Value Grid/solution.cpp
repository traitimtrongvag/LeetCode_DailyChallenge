class Solution {
public:
    int minOperations(vector<vector<int>>& grid, int x) {
        int m = grid.size(), n = grid[0].size();

        vector<int> a;
        a.reserve(m * n);

        // normalize values by dividing with x
        int rem = grid[0][0] % x;

        for (auto &row : grid) {
            for (int v : row) {
                if (v % x != rem) return -1; // cannot make equal
                a.push_back(v / x);
            }
        }

        // median gives minimum total distance
        sort(a.begin(), a.end());
        int mid = a[a.size() / 2];

        int ops = 0;
        for (int v : a) {
            ops += abs(v - mid);
        }

        return ops;
    }
};
