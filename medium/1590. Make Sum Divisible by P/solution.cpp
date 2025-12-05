class Solution {
public:
    int minSubarray(vector<int>& nums, int p) {
        long long total = 0;
        for (int num : nums) {
            total += num;
        }
        
        int target = total % p;
        if (target == 0) return 0;
        
        unordered_map<int, int> prefix;
        prefix[0] = -1;
        long long sum = 0;
        int minLen = nums.size();
        
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i];
            int curr = sum % p;
            int need = (curr - target + p) % p;
            
            if (prefix.count(need)) {
                minLen = min(minLen, i - prefix[need]);
            }
            prefix[curr] = i;
        }
        
        return minLen == nums.size() ? -1 : minLen;
    }
};