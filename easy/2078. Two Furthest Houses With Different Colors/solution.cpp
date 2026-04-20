class Solution {
public:
    int maxDistance(vector<int>& colors) {
        int n = colors.size();
        int ans = 0;

        // compare with first house
        for (int i = 0; i < n; i++) {
            if (colors[i] != colors[0]) {
                ans = max(ans, i);
            }
        }

        // compare with last house
        for (int i = n - 1; i >= 0; i--) {
            if (colors[i] != colors[n - 1]) {
                ans = max(ans, (n - 1) - i);
            }
        }

        return ans;
    }
};