// 1930. Unique Length-3 Palindromic Subsequences
class Solution {
public:
    int countPalindromicSubsequence(string s) {
        int n = s.size();
        vector<int> first(26, -1), last(26, -1);

        for (int i = 0; i < n; i++) {
            int c = s[i] - 'a';
            if (first[c] == -1) first[c] = i;
            last[c] = i;
        }

        int ans = 0;

        for (int a = 0; a < 26; a++) {
            int l = first[a], r = last[a];
            if (l != -1 && r - l > 1) {
                bool seen[26] = {0};
                for (int i = l + 1; i < r; i++) {
                    int b = s[i] - 'a';
                    if (!seen[b]) {
                        seen[b] = true;
                        ans++;
                    }
                }
            }
        }
        return ans;
    }
};