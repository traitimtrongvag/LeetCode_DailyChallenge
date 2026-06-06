/*
Time: O(n)
Space: O(n)

Idea:
Keep total sum on the right.

Move elements one by one:
- remove current value from right sum
- answer = abs(left - right)
- add current value to left sum
*/

class Solution {
public:
    vector<int> leftRightDifference(vector<int>& nums) {
        int n = nums.size();

        int rs = 0;
        for (int x : nums) {
            rs += x;
        }

        int ls = 0;
        vector<int> ans(n);

        for (int i = 0; i < n; i++) {
            rs -= nums[i];

            ans[i] = abs(ls - rs);

            ls += nums[i];
        }

        return ans;
    }
};