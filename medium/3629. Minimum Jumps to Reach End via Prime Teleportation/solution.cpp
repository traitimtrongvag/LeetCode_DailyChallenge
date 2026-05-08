/*
Time: O(n * sqrt(max(nums)))
Space: O(n)

Idea: BFS + prime factor graph
*/

class Solution {
public:
    vector<int> getFactors(int x, vector<int>& spf) {
        vector<int> res;

        while (x > 1) {
            int p = spf[x];
            res.push_back(p);

            while (x % p == 0) {
                x /= p;
            }
        }

        return res;
    }

    int minJumps(vector<int>& nums) {
        int n = nums.size();

        int mx = *max_element(nums.begin(), nums.end());

        // smallest prime factor
        vector<int> spf(mx + 1);

        for (int i = 2; i <= mx; i++) {
            if (spf[i] == 0) {
                spf[i] = i;

                if (1LL * i * i <= mx) {
                    for (long long j = 1LL * i * i; j <= mx; j += i) {
                        if (spf[j] == 0) {
                            spf[j] = i;
                        }
                    }
                }
            }
        }

        unordered_map<int, vector<int>> mp;
        vector<vector<int>> facs(n);

        // store factors for each index
        for (int i = 0; i < n; i++) {
            facs[i] = getFactors(nums[i], spf);

            for (int p : facs[i]) {
                mp[p].push_back(i);
            }
        }

        vector<bool> vis(n, false);

        queue<pair<int,int>> q;
        q.push({0, 0});
        vis[0] = true;

        while (!q.empty()) {
            auto [i, d] = q.front();
            q.pop();

            if (i == n - 1) {
                return d;
            }

            // move right
            if (i + 1 < n && !vis[i + 1]) {
                vis[i + 1] = true;
                q.push({i + 1, d + 1});
            }

            // move left
            if (i > 0 && !vis[i - 1]) {
                vis[i - 1] = true;
                q.push({i - 1, d + 1});
            }

            int v = nums[i];

            // jump using prime factors
            if (spf[v] == v) {
                for (int p : facs[i]) {
                    auto &list = mp[p];

                    for (int j : list) {
                        if (!vis[j]) {
                            vis[j] = true;
                            q.push({j, d + 1});
                        }
                    }

                    list.clear();
                }
            }
        }

        return -1;
    }
};