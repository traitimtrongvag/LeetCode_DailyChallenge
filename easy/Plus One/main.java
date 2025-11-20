// 66. Plus one
class Solution {
    public int[] plusOne(int[] digits) {
        int i = digits.length;
        while (i > 0) {
            i--;
            if (digits[i] < 9) {
                digits[i]++;
                return digits;
            }
            digits[i] = 0;
        }
        int[] r = new int[digits.length + 1];
        r[0] = 1;
        return r;
    }
}