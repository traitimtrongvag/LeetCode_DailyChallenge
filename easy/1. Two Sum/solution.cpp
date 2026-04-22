#include <vector>
#include <unordered_map>

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> seen; // value -> index
        
        for (int i = 0; i < nums.size(); i++) {
            int need = target - nums[i];
            
            // check if complement already exists
            if (seen.find(need) != seen.end()) {
                return {seen[need], i};
            }
            
            // store current value for future lookups
            seen[nums[i]] = i;
        }
        
        return {}; // should never reach here given constraints
    }
};