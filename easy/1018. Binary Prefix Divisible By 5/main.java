// 1018. Binary Prefix Divisible By 5
class Solution {
    public List<Boolean> prefixesDivBy5(int[] nums) {
        List<Boolean> result = new ArrayList<>();
        int current = 0;
        
        for (int num : nums) {
            current = (current * 2 + num) % 5;
            result.add(current == 0);
        }
        
        return result;
    }
}