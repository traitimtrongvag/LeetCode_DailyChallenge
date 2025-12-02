// 28. Find the Index of the First Occurrence in a String
class Solution {
    public int strStr(String haystack, String needle) {
        if (needle.isEmpty()) return 0;
        
        int m = haystack.length(), n = needle.length();
        for (int i = 0; i <= m - n; i++) {
            int j = 0;
            while (j < n && haystack.charAt(i + j) == needle.charAt(j)) {
                j++;
            }
            if (j == n) return i;
        }
        return -1;
    }
}