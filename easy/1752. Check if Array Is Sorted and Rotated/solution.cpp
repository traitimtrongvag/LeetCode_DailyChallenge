/*
Time: O(n)
Space: O(1)

Idea: count drops, valid array has at most one decrease
*/

class Solution {
public:
    bool check(vector<int>& nums) {
        int n = nums.size();

        int drops = 0;

        for (int i = 0; i < n; i++) {
            int next = (i + 1) % n;

            if (nums[i] > nums[next]) {
                drops++;
            }
        }

        return drops <= 1;
    }
};