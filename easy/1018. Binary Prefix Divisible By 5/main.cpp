// 1018. Binary Prefix Divisible By 5
class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& nums) {
        vector<bool> result;
        int current = 0;
        
        for (int num : nums) {
            current = (current * 2 + num) % 5;
            result.push_back(current == 0);
        }
        
        return result;
    }
};