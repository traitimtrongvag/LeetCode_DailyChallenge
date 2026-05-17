/*
Time: O(n)
Space: O(n)

Idea: DFS/BFS from start, visit reachable indices once
*/

class Solution {
public:
    bool canReach(vector<int>& arr, int start) {
        int n = arr.size();

        queue<int> q;
        vector<bool> vis(n, false);

        q.push(start);
        vis[start] = true;

        while (!q.empty()) {
            int i = q.front();
            q.pop();

            if (arr[i] == 0) {
                return true;
            }

            int left = i - arr[i];
            int right = i + arr[i];

            if (left >= 0 && !vis[left]) {
                vis[left] = true;
                q.push(left);
            }

            if (right < n && !vis[right]) {
                vis[right] = true;
                q.push(right);
            }
        }

        return false;
    }
};