class Solution {
    public int countPalindromicSubsequence(String s) {
        int n = s.length();
        int[] first = new int[26];
        int[] last = new int[26];

        Arrays.fill(first, -1);
        Arrays.fill(last, -1);

        for (int i = 0; i < n; i++) {
            int c = s.charAt(i) - 'a';
            if (first[c] == -1) first[c] = i;
            last[c] = i;
        }

        int ans = 0;

        for (int a = 0; a < 26; a++) {
            int l = first[a], r = last[a];
            if (l != -1 && r - l > 1) {
                boolean[] seen = new boolean[26];
                for (int i = l + 1; i < r; i++) {
                    int b = s.charAt(i) - 'a';
                    if (!seen[b]) {
                        seen[b] = true;
                        ans++;
                    }
                }
            }
        }
        return ans;
    }
}