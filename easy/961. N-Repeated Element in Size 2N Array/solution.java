class Solution {
    public int repeatedNTimes(int[] nums) {
        for (int i = 0; i < nums.length - 1; i++) {
            if (nums[i] == nums[i + 1]) return nums[i];
            if (i + 2 < n)
                if (nums[i] == nums[i + 2]) return nums[i];
        }
        
        return nums[0];
    }
}