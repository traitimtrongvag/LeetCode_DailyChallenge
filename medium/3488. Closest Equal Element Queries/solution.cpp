class Solution {
public:
    // compute circular distance between two indices
    int dist(int a, int b, int n) {
        int d = abs(a - b);
        return min(d, n - d);
    }

    vector<int> solveQueries(vector<int>& nums, vector<int>& queries) {
        int n = nums.size();

        // map value -> list of indices where it appears
        unordered_map<int, vector<int>> pos;
        for (int i = 0; i < n; i++) {
            pos[nums[i]].push_back(i);
        }

        vector<int> ans;
        ans.reserve(queries.size());

        for (int idx : queries) {
            int val = nums[idx];
            auto& v = pos[val];

            // only one occurrence -> no valid answer
            if (v.size() <= 1) {
                ans.push_back(-1);
                continue;
            }

            // find current index position in v
            int p = lower_bound(v.begin(), v.end(), idx) - v.begin();

            // nearest neighbors in circular index list
            int left = v[(p - 1 + v.size()) % v.size()];
            int right = v[(p + 1) % v.size()];

            // take best circular distance
            int best = min(dist(idx, left, n), dist(idx, right, n));

            ans.push_back(best);
        }

        return ans;
    }
};