class Solution {
public:
    string mapWordWeights(vector<string>& words, vector<int>& weights) {
        int n = weights.size();
        string res = "";
        for (string str : words) {
            int sum = 0;
            for (char ch : str) {
                sum += weights[ch - 'a'];
            }
            res += (char)('a' + (n - (sum % 26) - 1 + 26) % 26);
        }
        return res;
    }
};