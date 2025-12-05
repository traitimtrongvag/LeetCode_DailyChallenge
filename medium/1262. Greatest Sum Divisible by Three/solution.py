class Solution:
    def maxSumDivThree(self, nums: List[int]) -> int:
        dp = [0, -10**9, -10**9]
        
        for num in nums:
            temp = dp[:]
            for i in range(3):
                remainder = (temp[i] + num) % 3
                dp[remainder] = max(dp[remainder], temp[i] + num)
        
        return dp[0]