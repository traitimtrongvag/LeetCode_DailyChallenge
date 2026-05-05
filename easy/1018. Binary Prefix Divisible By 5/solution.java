/*
Time: O(n)
Space: O(1) (excluding output)

Idea: build binary prefix mod 5, only keep remainder
*/

class Solution {
    public List<Boolean> prefixesDivBy5(int[] nums) {
        List<Boolean> res = new ArrayList<>();
        int cur = 0; // current value mod 5
        
        for (int bit : nums) {
            // shift left (x2) and add new bit, keep mod 5
            cur = (cur * 2 + bit) % 5;

            // divisible by 5 if remainder == 0
            res.add(cur == 0);
        }
        
        return res;
    }
}