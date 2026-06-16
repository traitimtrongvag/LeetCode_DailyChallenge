class Solution {
public:
    string processStr(string s) {
        string result = "";

        for (char d : s) {
            if (d == '*') {
                if (!result.empty()) result.pop_back();
            } else if (d == '#') {
                result += result;
            } else if (d == '%') {
                reverse(result.begin(), result.end());
            } else {
                result += d;
            }
        }

        return result;
    }
};