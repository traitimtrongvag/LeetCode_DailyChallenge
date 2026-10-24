class Solution(object):
    def prefixesDivBy5(self, nums):
        answer = []
        remainder = 0

        for num in nums:
            remainder = (remainder * 2 + num) % 5
            answer.append(remainder == 0)

        return answer
