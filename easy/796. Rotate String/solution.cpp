/*
KMP approach for rotate string

Idea:
- A rotation of string s must be a substring of (s + s)
- Instead of using built-in find(), use KMP for stable O(n)

Steps:
1. Build LPS array for pattern (goal)
2. Run KMP on (s + s) to check if goal exists
3. If found -> true, else false

Time: O(n)
Space: O(n)
*/

class Solution {
public:
    vector<int> buildLPS(string &p) {
        int n = p.size();
        vector<int> lps(n, 0);

        for (int i = 1, j = 0; i < n; i++) {
            while (j > 0 && p[i] != p[j]) {
                j = lps[j - 1];
            }
            if (p[i] == p[j]) j++;
            lps[i] = j;
        }
        return lps;
    }

    bool kmp(string &s, string &p) {
        vector<int> lps = buildLPS(p);

        for (int i = 0, j = 0; i < s.size(); i++) {
            while (j > 0 && s[i] != p[j]) {
                j = lps[j - 1];
            }
            if (s[i] == p[j]) j++;
            if (j == p.size()) return true;
        }
        return false;
    }

    bool rotateString(string s, string goal) {
        if (s.size() != goal.size()) return false;

        string t = s + s;
        return kmp(t, goal);
    }
};