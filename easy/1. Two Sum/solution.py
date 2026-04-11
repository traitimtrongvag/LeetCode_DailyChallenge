class Solution(object):
    def twoSum(self, nums, target):
        num_map = {}
        for i, num in enumerate(nums):
            need = target - num
            if need in num_map:
                return [num_map[need], i]
            num_map[num] = i