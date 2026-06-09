class Solution {
public:
    long long maxTotalValue(vector<int>& nums, int k) {
        long long maxVal = nums[0];
        long long minVal = nums[0];
        
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] > maxVal) maxVal = nums[i];
            if (nums[i] < minVal) minVal = nums[i];
        }
        
        return k * (maxVal - minVal);
    }
};