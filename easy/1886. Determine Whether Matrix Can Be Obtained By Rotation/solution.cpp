/*
Time: O(n^2)
Space: O(1)

Idea: rotate matrix up to 4 times, check if equal to target
*/

class Solution {
public:
    void rotate(vector<vector<int>>& m) {
        int n = m.size();

        // transpose
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                swap(m[i][j], m[j][i]);
            }
        }

        // reverse rows
        for (int i = 0; i < n; i++) {
            reverse(m[i].begin(), m[i].end());
        }
    }

    bool same(vector<vector<int>>& a, vector<vector<int>>& b) {
        return a == b;
    }

    bool findRotation(vector<vector<int>>& mat, vector<vector<int>>& target) {
        for (int i = 0; i < 4; i++) {
            if (same(mat, target)) {
                return true;
            }

            rotate(mat);
        }

        return false;
    }
};