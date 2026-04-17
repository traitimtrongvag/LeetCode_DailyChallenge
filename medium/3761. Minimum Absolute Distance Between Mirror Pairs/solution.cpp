class Solution {
public:
    // reverse digits
    int rev(int x) {
        int y = 0;
        while (x) {
            y = y * 10 + x % 10;
            x /= 10;
        }
        return y;
    }

    int minMirrorPairDistance(vector<int>& nums) {
        int n = nums.size();
        unordered_map<int, int> last; // reversed value -> last index
        int ans = n + 1;

        for (int i = 0; i < n; i++) {
            int x = nums[i];

            // check if current matches some previous reversed
            if (last.count(x)) {
                ans = min(ans, i - last[x]);
            }

            // update reversed of current
            last[rev(x)] = i;
        }

        return ans == n + 1 ? -1 : ans;
    }
};