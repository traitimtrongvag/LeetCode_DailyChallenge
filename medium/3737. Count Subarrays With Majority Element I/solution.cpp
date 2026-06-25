class Solution {
public:
    int countMajoritySubarrays(vector<int>& nums, int target) {
        int n = nums.size(), ans = 0;

        for (int i = 0; i < n; i++) {
            int cnt = 0;

            for (int j = i; j < n; j++) {
                if (nums[j] == target) cnt++;

                if (cnt * 2 > j - i + 1)
                    ans++;
            }
        }

        return ans;
    }
};