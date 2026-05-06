/*
Time: O(m * n)
Space: O(m * n)

Idea: simulate gravity row by row (stones fall to the right), then rotate 90°
*/

class Solution {
public:
    vector<vector<char>> rotateTheBox(vector<vector<char>>& box) {
        int m = box.size(), n = box[0].size();

        // apply gravity: stones '#' fall to the right
        for (int i = 0; i < m; i++) {
            int write = n - 1; // position to drop next stone

            for (int j = n - 1; j >= 0; j--) {
                if (box[i][j] == '*') {
                    write = j - 1; // reset after obstacle
                } else if (box[i][j] == '#') {
                    swap(box[i][j], box[i][write]);
                    write--;
                }
            }
        }

        // rotate 90° clockwise
        vector<vector<char>> res(n, vector<char>(m));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                res[j][m - 1 - i] = box[i][j];
            }
        }

        return res;
    }
};