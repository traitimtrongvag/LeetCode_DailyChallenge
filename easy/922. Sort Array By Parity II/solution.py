class Solution:
    def sortArrayByParityII(self, nums: List[int]) -> List[int]:
        even = 0
        odd = 1
        
        while even < len(nums) and odd < len(nums):
            if nums[even] % 2 == 0:
                even += 2
            else:
                nums[even], nums[odd] = nums[odd], nums[even]
                odd += 2
        
        return nums