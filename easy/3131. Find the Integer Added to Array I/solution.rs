// Algorithm overview:
// The difference added to all elements is consistent.
// The added value can be found by comparing a representative value
// from both arrays. Using the maximum element works because the shift
// applies equally to every element.
// Compute max(nums2) - max(nums1) to get the added integer.

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // The added integer is the difference between the maximum elements.
        nums2.iter().max().unwrap() - nums1.iter().max().unwrap()
    }
}