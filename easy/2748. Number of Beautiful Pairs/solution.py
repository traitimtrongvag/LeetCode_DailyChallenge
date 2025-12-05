class Solution:
    def countBeautifulPairs(self, nums: List[int]) -> int:
        def first_digit(num):
            while num >= 10:
                num //= 10
            return num
        
        def gcd(a, b):
            if b == 0:
                return a
            return gcd(b, a % b)
        
        count = 0
        n = len(nums)
        
        for i in range(n):
            for j in range(i + 1, n):
                first = first_digit(nums[i])
                last = nums[j] % 10
                
                if gcd(first, last) == 1:
                    count += 1
        
        return count