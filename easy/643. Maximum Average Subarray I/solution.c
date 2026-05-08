/*
Time: O(n)
Space: O(1)

Idea: sliding window of size k
*/

double findMaxAverage(int* nums, int numsSize, int k) {
    long long sum = 0;

    // first window
    for (int i = 0; i < k; i++) {
        sum += nums[i];
    }

    long long best = sum;

    // slide window
    for (int i = k; i < numsSize; i++) {
        sum += nums[i] - nums[i - k];

        if (sum > best) {
            best = sum;
        }
    }

    return (double)best / k;
}