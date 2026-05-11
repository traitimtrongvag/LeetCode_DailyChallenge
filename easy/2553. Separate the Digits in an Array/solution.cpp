/*
Time: O(total digits)
Space: O(total digits)

Idea: split each number into digits in correct order
*/

class Solution {
public:
    vector<int> separateDigits(vector<int>& nums) {
        vector<int> ans;

        for (int x : nums) {
            string s = to_string(x);

            for (char c : s) {
                ans.push_back(c - '0');
            }
        }

        return ans;
    }
};