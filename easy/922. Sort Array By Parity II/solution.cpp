class Solution {
public:
    vector<int> sortArrayByParityII(vector<int>& nums) {
        int even = 0;
        int odd = 1;
        
        while (even < nums.size() && odd < nums.size()) {
            if (nums[even] % 2 == 0) {
                even += 2;
            } else {
                swap(nums[even], nums[odd]);
                odd += 2;
            }
        }
        
        return nums;
    }
};