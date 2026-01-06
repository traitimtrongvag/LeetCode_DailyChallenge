class Solution {
    public int thirdMax(int[] nums) {
        Arrays.sort(nums);
        Set<Integer> seen = new HashSet<>();

        for (int i = nums.length - 1; i >= 0; i--) {
            seen.add(nums[i]);
            if (seen.size() == 3) {
                return nums[i];
            }
        }

        return nums[nums.length - 1];
    }
}
