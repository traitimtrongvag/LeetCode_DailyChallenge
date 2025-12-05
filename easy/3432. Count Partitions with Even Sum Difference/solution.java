// 3432. Count Partitions with Even Sum Difference
class Solution {
    public int countPartitions(int[] nums) {
        int total = 0;
        for (int x : nums) total += x;
        int left = 0, ans = 0;
        for (int i = 0; i < nums.length - 1; i++) {
            left += nums[i];
            int right = total - left;
            if (((left - right) & 1) == 0) ans++;
        }
        return ans;
    }
}