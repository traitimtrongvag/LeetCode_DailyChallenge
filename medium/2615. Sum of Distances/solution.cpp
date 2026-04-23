class Solution {
public:
    vector<long long> distance(vector<int>& nums) {
        int n = nums.size();
        vector<long long> ans(n, 0);

        unordered_map<int, long long> cnt, sum;

        // left -> right
        for (int i = 0; i < n; i++) {
            int x = nums[i];

            if (cnt.count(x)) {
                // contribution from left side
                ans[i] += cnt[x] * i - sum[x];
            }

            cnt[x]++;
            sum[x] += i;
        }

        cnt.clear();
        sum.clear();

        // right -> left
        for (int i = n - 1; i >= 0; i--) {
            int x = nums[i];

            if (cnt.count(x)) {
                // contribution from right side
                ans[i] += sum[x] - cnt[x] * i;
            }

            cnt[x]++;
            sum[x] += i;
        }

        return ans;
    }
};