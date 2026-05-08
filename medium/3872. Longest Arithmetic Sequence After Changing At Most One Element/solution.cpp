/*
Time: O(n)
Space: O(n)

Idea: precompute arithmetic lengths from left/right,
try changing one element to extend sequence
*/

class Solution {
public:
    int longestArithmetic(vector<int>& nums) {
        int n = nums.size();

        if (n <= 2) return n;

        int ans = 2;

        vector<int> left(n, 2);
        vector<int> right(n, 2);

        // longest arithmetic subarray ending at i
        for (int i = 2; i < n; i++) {
            if (nums[i] - nums[i - 1] ==
                nums[i - 1] - nums[i - 2]) {

                left[i] = left[i - 1] + 1;
            }
        }

        // longest arithmetic subarray starting at i
        for (int i = n - 3; i >= 0; i--) {
            if (nums[i + 2] - nums[i + 1] ==
                nums[i + 1] - nums[i]) {

                right[i] = right[i + 1] + 1;
            }
        }

        for (int i = 0; i < n; i++) {
            ans = max(ans, max(left[i], right[i]));

            // change edge element
            if (i == 0) {
                ans = max(ans, 1 + right[i + 1]);
            }
            else if (i == n - 1) {
                ans = max(ans, 1 + left[i - 1]);
            }
            else {
                ans = max(ans, 1 + left[i - 1]);
                ans = max(ans, 1 + right[i + 1]);

                int diff = nums[i + 1] - nums[i - 1];

                // middle value must exist
                if (diff % 2 == 0) {
                    int d = diff / 2;

                    int L = 1;
                    int R = 1;

                    if (i >= 2 &&
                        nums[i - 1] - nums[i - 2] == d) {

                        L = left[i - 1];
                    }

                    if (i < n - 2 &&
                        nums[i + 2] - nums[i + 1] == d) {

                        R = right[i + 1];
                    }

                    ans = max(ans, 1 + L + R);
                }
            }
        }

        return ans;
    }
};