/*
Time: O(n)
Space: O(n)

Idea: two pointers, place larger square from back
*/

class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        int n = nums.size();

        vector<int> ans(n);

        int l = 0;
        int r = n - 1;

        int pos = n - 1;

        while (l <= r) {
            int a = nums[l] * nums[l];
            int b = nums[r] * nums[r];

            if (a > b) {
                ans[pos--] = a;
                l++;
            } else {
                ans[pos--] = b;
                r--;
            }
        }

        return ans;
    }
};