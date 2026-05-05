/*
Time: O(n)
Space: O(1) (excluding output)

Idea: build binary prefix mod 5, only keep remainder
*/

class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& nums) {
        vector<bool> result;
        int cur = 0; // current value mod 5
        
        for (int bit : nums) {
            // shift left (x2) and add new bit, keep mod 5
            cur = (cur * 2 + bit) % 5;

            // divisible by 5 if remainder == 0
            result.push_back(cur == 0);
        }
        
        return result;
    }
};