// 3381. Maximum Subarray Sum With Length Divisible by K
long long maxSubarraySum(int* nums, int numsSize, int k) {
    int n = numsSize;
    long long* prefix = (long long*)malloc((n + 1) * sizeof(long long));
    prefix[0] = 0;
    for (int i = 0; i < n; i++) {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    
    long long* minPrefix = (long long*)malloc(k * sizeof(long long));
    for (int i = 0; i < k; i++) {
        minPrefix[i] = LLONG_MAX;
    }
    minPrefix[0] = 0;
    long long maxSum = LLONG_MIN;
    
    for (int i = 1; i <= n; i++) {
        int rem = i % k;
        if (minPrefix[rem] != LLONG_MAX) {
            if (prefix[i] - minPrefix[rem] > maxSum) {
                maxSum = prefix[i] - minPrefix[rem];
            }
        }
        if (prefix[i] < minPrefix[rem]) {
            minPrefix[rem] = prefix[i];
        }
    }
    
    free(prefix);
    free(minPrefix);
    return maxSum;
}