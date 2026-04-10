class Solution {
public:
    int tempMINdistance(vector<int> nums, int target)
    {
        vector<int> temp;
        for(int i = 0; i < nums.size(); i++)
        {
            if(nums[i] == target)
                temp.push_back(i);
        }

        int ans = INT_MAX;
        for(int i = 0; i <= temp.size() - 3; i++)
        {
            int t = abs(temp[i] - temp[i+1]) 
                  + abs(temp[i+1] - temp[i+2]) 
                  + abs(temp[i+2] - temp[i]);

            ans = min(ans, t);
        }

        return ans;
    }

    int minimumDistance(vector<int>& nums) {
        map<int,int> freq;
        vector<int> ans;

        for(int i = 0; i < nums.size(); i++)
        {
            freq[nums[i]]++;
            if(freq[nums[i]] == 3)
                ans.push_back(tempMINdistance(nums, nums[i]));
        }

        if(ans.empty()) return -1;
        return *min_element(ans.begin(), ans.end());
    }
};