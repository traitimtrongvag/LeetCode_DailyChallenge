/*
Time: O(n)
Space: O(n)

Idea: BFS + value to indices map, clear used groups to avoid repeats
*/

class Solution {
public:
    int minJumps(vector<int>& arr) {
        int n = arr.size();

        if (n == 1) return 0;

        unordered_map<int, vector<int>> mp;

        for (int i = 0; i < n; i++) {
            mp[arr[i]].push_back(i);
        }

        queue<int> q;
        vector<bool> vis(n, false);

        q.push(0);
        vis[0] = true;

        int steps = 0;

        while (!q.empty()) {
            int sz = q.size();

            while (sz--) {
                int i = q.front();
                q.pop();

                if (i == n - 1) {
                    return steps;
                }

                // left
                if (i - 1 >= 0 && !vis[i - 1]) {
                    vis[i - 1] = true;
                    q.push(i - 1);
                }

                // right
                if (i + 1 < n && !vis[i + 1]) {
                    vis[i + 1] = true;
                    q.push(i + 1);
                }

                // same value jumps
                for (int j : mp[arr[i]]) {
                    if (!vis[j]) {
                        vis[j] = true;
                        q.push(j);
                    }
                }

                // prevent reprocessing
                mp[arr[i]].clear();
            }

            steps++;
        }

        return -1;
    }
};