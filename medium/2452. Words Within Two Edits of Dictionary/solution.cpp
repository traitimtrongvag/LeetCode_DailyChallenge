class Solution {
public:
    vector<string> twoEditWords(vector<string>& queries, vector<string>& dictionary) {
        vector<string> result;
        int wordLen = queries[0].size(); // all words have same length
        
        for (int i = 0; i < queries.size(); i++) {
            string& queryWord = queries[i];
            bool found = false;
            
            // check each dictionary word
            for (string& dictWord : dictionary) {
                int diffCount = 0;
                // compare character by character
                for (int pos = 0; pos < wordLen; pos++) {
                    if (queryWord[pos] != dictWord[pos]) {
                        diffCount++;
                    }
                    // early stop if already more than 2 edits
                    if (diffCount > 2) {
                        break;
                    }
                }
                // if within 2 edits, add to result and stop checking other dict words
                if (diffCount <= 2) {
                    result.push_back(queryWord);
                    found = true;
                    break;
                }
            }
        }
        return result;
    }
};