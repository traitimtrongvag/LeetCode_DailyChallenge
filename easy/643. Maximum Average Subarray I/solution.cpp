/*
Time: O(n)
Space: O(1)

Idea: sliding window of size k
*/

class Solution {
public:
    double findMaxAverage(vector<int>& nums, int k) {
        long long sum = 0;

        // first window
        for (int i = 0; i < k; i++) {
            sum += nums[i];
        }

        long long best = sum;

        // slide window
        for (int i = k; i < nums.size(); i++) {
            sum += nums[i] - nums[i - k];
            best = max(best, sum);
        }

        return (double)best / k;
    }
};