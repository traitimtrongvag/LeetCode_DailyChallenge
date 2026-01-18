// Algorithm overview:
// All elements in nums1 are shifted by the same value to form nums2.
// That value can be determined by comparing the minimum elements.
// Compute the minimum of both arrays and return their difference.

class Solution {
    public int addedInteger(int[] nums1, int[] nums2) {
        // Track the minimum value in nums1.
        int min1 = Integer.MAX_VALUE;
        // Track the minimum value in nums2.
        int min2 = Integer.MAX_VALUE;

        // Find the minimum values in both arrays.
        for (int i = 0; i < nums1.length; i++) {
            min1 = nums1[i] < min1 ? nums1[i] : min1;
            min2 = nums2[i] < min2 ? nums2[i] : min2;
        }

        // The added integer is the difference between the minimum values.
        return min2 - min1;
    }
}