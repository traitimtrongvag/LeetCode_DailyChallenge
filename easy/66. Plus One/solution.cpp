class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int i = digits.size();
        while (i > 0) {
            i--;
            if (digits[i] < 9) {
                digits[i]++;
                return digits;
            }
            digits[i] = 0;
        }
        vector<int> r = {1};
        r.insert(r.end(), digits.begin(), digits.end());
        return r;
    }
};