class Solution {
public:
    int maxDistance(vector<int>& nums1, vector<int>& nums2) {
        int i = 0, j = 0;
        int n = nums1.size(), m = nums2.size();
        int ans = 0;

        while (i < n && j < m) {
            if (nums1[i] <= nums2[j]) {
                // valid pair, try extend j
                ans = max(ans, j - i);
                j++;
            } else {
                // nums1[i] too big, move i
                i++;
            }
        }

        return ans;
    }
};