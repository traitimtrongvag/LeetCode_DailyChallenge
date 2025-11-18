class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        num_map = {}  # Dictionary to store number and its index
        for i, num in enumerate(nums):
            complement = target - num  # Find the complement of the current number
            if complement in num_map:  # If complement exists in the dictionary
                return [num_map[complement], i]  # Return the indices
            num_map[num] = i  # Otherwise, store the current number and its index