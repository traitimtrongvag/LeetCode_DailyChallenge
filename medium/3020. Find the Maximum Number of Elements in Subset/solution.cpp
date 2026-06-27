class Solution {
public:
    int maximumLength(vector<int>& nums) {
        unordered_map<int,int> mp;
        for (auto x : nums) mp[x]++;

        int ans = 1;

        for (auto x : nums) {
            if (x == 1) {
                ans = max(ans, mp[1] - !(mp[1] & 1));
                continue;
            }

            if (mp[x] < 2) continue;

            long long cur = x;
            int len = 0;

            while (cur <= INT_MAX && mp.count(cur)) {
                len++;

                if (mp[cur] == 1) break;

                if (cur > LLONG_MAX / cur) break;
                cur *= cur;
            }

            ans = max(ans, len * 2 - 1);
        }

        return ans;
    }
};