/*
Time: O(n)
Space: O(1)

Idea: store lowercase and uppercase existence,
then count letters that appear in both forms
*/

class Solution {
public:
    int numberOfSpecialChars(string word) {
        vector<bool> lowerExist(26, false);
        vector<bool> upperExist(26, false);

        // record all characters
        for (char ch : word) {
            if (ch >= 'a' && ch <= 'z') {
                lowerExist[ch - 'a'] = true;
            } 
            else if (ch >= 'A' && ch <= 'Z') {
                upperExist[ch - 'A'] = true;
            }
        }

        int specialCount = 0;

        // a character is special if both lower and upper exist
        for (int i = 0; i < 26; i++) {
            if (lowerExist[i] && upperExist[i]) {
                specialCount++;
            }
        }

        return specialCount;
    }
};