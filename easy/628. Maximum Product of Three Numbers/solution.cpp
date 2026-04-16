class Solution {
public:
    int maximumProduct(vector<int>& nums) {
        // track top 3 max and 2 min
        int max1 = INT_MIN, max2 = INT_MIN, max3 = INT_MIN;
        int min1 = INT_MAX, min2 = INT_MAX;

        for (int x : nums) {
            // update max values
            if (x > max1) {
                max3 = max2;
                max2 = max1;
                max1 = x;
            } else if (x > max2) {
                max3 = max2;
                max2 = x;
            } else if (x > max3) {
                max3 = x;
            }

            // update min values
            if (x < min1) {
                min2 = min1;
                min1 = x;
            } else if (x < min2) {
                min2 = x;
            }
        }

        // case1: 3 largest
        int p1 = max1 * max2 * max3;

        // case2: 2 smallest (neg) * largest
        int p2 = max1 * min1 * min2;

        return max(p1, p2);
    }
};