/*
Time: O(log n) average, O(n) worst
Space: O(1)

Idea: binary search, shrink right when duplicates appear
*/

class Solution {
public:
    int findMin(vector<int>& nums) {
        int l = 0;
        int r = nums.size() - 1;

        while (l < r) {
            int mid = l + (r - l) / 2;

            if (nums[mid] < nums[r]) {
                r = mid;
            }
            else if (nums[mid] > nums[r]) {
                l = mid + 1;
            }
            else {
                r--;
            }
        }

        return nums[l];
    }
};