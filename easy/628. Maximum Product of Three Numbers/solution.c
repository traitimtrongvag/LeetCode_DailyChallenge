#include <limits.h>

int maximumProduct(int* nums, int numsSize) {
    int max1 = INT_MIN, max2 = INT_MIN, max3 = INT_MIN;
    int min1 = INT_MAX, min2 = INT_MAX;

    for (int i = 0; i < numsSize; i++) {
        int x = nums[i];

        // update 3 largest
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

        // update 2 smallest
        if (x < min1) {
            min2 = min1;
            min1 = x;
        } else if (x < min2) {
            min2 = x;
        }
    }

    // max of:
    // 1) 3 largest
    // 2) largest * 2 smallest (neg case)
    int p1 = max1 * max2 * max3;
    int p2 = max1 * min1 * min2;

    return p1 > p2 ? p1 : p2;
}