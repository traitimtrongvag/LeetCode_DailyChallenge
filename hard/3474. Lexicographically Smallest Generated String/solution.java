class Solution {
    public String generateString(String str1, String str2) {
        int n = str1.length(), m = str2.length();
        char[] s = new char[n + m - 1];
        boolean[] fixed = new boolean[n + m - 1];

        Arrays.fill(s, 'a');

        // force match where str1[i] == 'T'
        for (int i = 0; i < n; i++) {
            if (str1.charAt(i) == 'T') {
                for (int j = 0; j < m; j++) {
                    int pos = i + j;
                    if (fixed[pos] && s[pos] != str2.charAt(j)) return "";
                    s[pos] = str2.charAt(j);
                    fixed[pos] = true;
                }
            }
        }

        // break match where str1[i] == 'F'
        for (int i = 0; i < n; i++) {
            if (str1.charAt(i) == 'F') {
                boolean diff = false;
                int change = -1;

                for (int j = 0; j < m; j++) {
                    int pos = i + j;
                    if (s[pos] != str2.charAt(j)) diff = true;
                    if (!fixed[pos]) change = pos;
                }

                if (diff) continue;           // already different
                if (change != -1) s[change] = 'b'; // break it
                else return "";               // cannot break
            }
        }

        return new String(s);
    }
}