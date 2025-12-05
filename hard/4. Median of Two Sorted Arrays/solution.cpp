class Solution {
public:
    double findMedianSortedArrays(vector<int>& a, vector<int>& b) {
        if (a.size() > b.size()) swap(a,b);
        int m = a.size(), n = b.size();
        int half = (m + n + 1) / 2;
        int l = 0, r = m;
        while (l <= r) {
            int i = l + (r - l) / 2;
            int j = half - i;
            int aLeft = (i == 0) ? INT_MIN : a[i-1];
            int aRight = (i == m) ? INT_MAX : a[i];
            int bLeft = (j == 0) ? INT_MIN : b[j-1];
            int bRight = (j == n) ? INT_MAX : b[j];
            if (aLeft <= bRight && bLeft <= aRight) {
                if ((m + n) % 2 == 1) return (double)max(aLeft, bLeft);
                return ((double)max(aLeft, bLeft) + (double)min(aRight, bRight)) / 2.0;
            } else if (aLeft > bRight) {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
        return 0.0;
    }
};