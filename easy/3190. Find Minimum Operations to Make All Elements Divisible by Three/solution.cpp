class Solution {
public:
    int minimumOperations(vector<int>& nums) {
        int operations = 0;
        for (int num : nums) {
            int remainder = num % 3;
            operations += min(remainder, 3 - remainder);
        }
        return operations;
    }
};