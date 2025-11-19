# 53. Maximum Subarray
class Solution:
    def maxSubArray(self, nums):
        current_sum = 0
        max_sum = nums[0]

        for x in nums:
            current_sum = max(x, current_sum + x)
            max_sum = max(max_sum, current_sum)

        return max_sum