/*
Time: O(n * m)
Space: O(1)

Idea: count required letters, check each word
*/

class Solution {
public:
    bool ok(vector<int>& need, string& word) {
        vector<int> cnt(26);

        for (char c : word) {
            cnt[tolower(c) - 'a']++;
        }

        for (int i = 0; i < 26; i++) {
            if (cnt[i] < need[i]) {
                return false;
            }
        }

        return true;
    }

    string shortestCompletingWord(string licensePlate, vector<string>& words) {
        vector<int> need(26);

        // count required letters
        for (char c : licensePlate) {
            if (isalpha(c)) {
                need[tolower(c) - 'a']++;
            }
        }

        string ans;

        for (string& w : words) {
            if (!ok(need, w)) continue;

            if (ans.empty() || w.size() < ans.size()) {
                ans = w;
            }
        }

        return ans;
    }
};