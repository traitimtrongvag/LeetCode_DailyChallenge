// 1590. Make Sum Divisible by P
class Solution {
    public int minSubarray(int[] nums, int p) {
        long total = 0;
        for (int num : nums) {
            total += num;
        }
        
        int target = (int)(total % p);
        if (target == 0) return 0;
        
        Map<Integer, Integer> prefix = new HashMap<>();
        prefix.put(0, -1);
        long sum = 0;
        int minLen = nums.length;
        
        for (int i = 0; i < nums.length; i++) {
            sum += nums[i];
            int curr = (int)(sum % p);
            int need = (curr - target + p) % p;
            
            if (prefix.containsKey(need)) {
                minLen = Math.min(minLen, i - prefix.get(need));
            }
            prefix.put(curr, i);
        }
        
        return minLen == nums.length ? -1 : minLen;
    }
}