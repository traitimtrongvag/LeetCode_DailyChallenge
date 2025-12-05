class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        i = len(digits)
        while i > 0:
            i -= 1
            if digits[i] < 9:
                digits[i] += 1
                return digits
            digits[i] = 0
        return [1] + digits