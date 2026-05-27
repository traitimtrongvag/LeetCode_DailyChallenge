/*
Time: O(n)
Space: O(1)

Idea: for each letter,
- remember last position of lowercase
- remember first position of uppercase

A character is special if:
last lowercase position < first uppercase position
*/

class Solution {
public:
    int numberOfSpecialChars(string word) {
        vector<int> lastLower(26, -1);
        vector<int> firstUpper(26, -1);

        for (int i = 0; i < word.size(); i++) {
            char ch = word[i];

            if (ch >= 'a' && ch <= 'z') {
                // keep updating because we need last occurrence
                lastLower[ch - 'a'] = i;
            }
            else {
                int index = ch - 'A';

                // only keep first occurrence
                if (firstUpper[index] == -1) {
                    firstUpper[index] = i;
                }
            }
        }

        int specialCount = 0;

        for (int i = 0; i < 26; i++) {
            // both lower and upper must exist
            if (lastLower[i] != -1 && firstUpper[i] != -1) {

                // lowercase must appear before uppercase
                if (lastLower[i] < firstUpper[i]) {
                    specialCount++;
                }
            }
        }

        return specialCount;
    }
};