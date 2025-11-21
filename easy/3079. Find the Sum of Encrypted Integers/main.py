# 3079. Find the Sum of Encrypted Integers
class Solution:
    def sumOfEncryptedInt(self, nums: List[int]) -> int:
        def encrypt(num: int) -> int:
            if num == 0:
                return 0
                
            max_digit = 0
            count = 0
            n = num
            
            # Find the maximum digit and count digits
            while n > 0:
                digit = n % 10
                if digit > max_digit:
                    max_digit = digit
                count += 1
                n //= 10
            
            # Build the encrypted number
            return int(str(max_digit) * count)
        
        total = 0
        for num in nums:
            total += encrypt(num)
        
        return total