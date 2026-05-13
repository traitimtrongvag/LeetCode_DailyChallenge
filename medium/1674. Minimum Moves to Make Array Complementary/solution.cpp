/*
Time: O(n + limit)
Space: O(limit)

Idea: difference array for range updates of move counts
*/

class Solution {
public:
    int minMoves(vector<int>& nums, int limit) {
        vector<int> diff(2 * limit + 2);

        int n = nums.size();

        for (int i = 0; i < n / 2; i++) {
            int a = nums[i];
            int b = nums[n - 1 - i];

            int low = min(a, b) + 1;
            int high = max(a, b) + limit;
            int sum = a + b;

            // default: 2 moves
            diff[2] += 2;

            // one move range
            diff[low] -= 1;
            diff[high + 1] += 1;

            // zero move at exact sum
            diff[sum] -= 1;
            diff[sum + 1] += 1;
        }

        int ans = INT_MAX;
        int cur = 0;

        for (int s = 2; s <= 2 * limit; s++) {
            cur += diff[s];
            ans = min(ans, cur);
        }

        return ans;
    }
};