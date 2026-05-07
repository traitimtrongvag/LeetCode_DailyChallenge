/*
Time: O(n)
Space: O(n)

Idea: keep prefix maximums, process segments recursively
*/

class Solution {
public:
    vector<int> maxValue(vector<int>& nums) {
        int n = nums.size();
        vector<int> ans(n);

        using P = pair<int,int>;

        // pref[i] = {max value so far, its index}
        vector<P> pref(n);

        P best = {INT_MIN, -1};

        for (int i = 0; i < n; i++) {
            if (nums[i] > best.first) {
                best = {nums[i], i};
            }
            pref[i] = best;
        }

        auto dfs = [&](auto&& self, int r, int mn, int mx) -> void {
            auto [val, pos] = pref[r];

            int cur = (val <= mn ? val : mx);

            int nextMin = min(mn, val);

            // fill current segment
            for (int i = pos; i <= r; i++) {
                ans[i] = cur;
                nextMin = min(nextMin, nums[i]);
            }

            if (pos == 0) return;

            self(self, pos - 1, nextMin, cur);
        };

        dfs(dfs, n - 1, INT_MAX, 0);

        return ans;
    }
};