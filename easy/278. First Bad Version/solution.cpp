class Solution {
public:
    int firstBadVersion(int n) {
        int l = 1, h = n;
        int ans = n;

        while (l <= h) {
            int mid = l + (h - l) / 2;

            if (!isBadVersion(mid)) {
                l = mid + 1;
            } else {
                ans = mid;
                h = mid - 1;
            }
        }
        return ans;
    }
};