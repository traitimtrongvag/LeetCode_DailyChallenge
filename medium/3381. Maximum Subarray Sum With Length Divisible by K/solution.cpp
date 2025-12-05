class Solution {
public:
    long long maxSubarraySum(vector<int>& nums, int k) {
        int n = nums.size();
        vector<long long> prefix(n + 1, 0);
        for (int i = 0; i < n; i++) {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        
        vector<long long> minPrefix(k, LLONG_MAX);
        minPrefix[0] = 0;
        long long maxSum = LLONG_MIN;
        
        for (int i = 1; i <= n; i++) {
            int rem = i % k;
            if (minPrefix[rem] != LLONG_MAX) {
                maxSum = max(maxSum, prefix[i] - minPrefix[rem]);
            }
            minPrefix[rem] = min(minPrefix[rem], prefix[i]);
        }
        
        return maxSum;
    }
};