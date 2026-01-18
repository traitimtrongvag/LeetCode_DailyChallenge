// Algorithm overview:
// Use a monotonic decreasing stack to process nums2.
// When a larger number appears, it becomes the next greater element
// for all smaller numbers popped from the stack.
// Store these relations in a hash map for fast lookup.
// Finally, build the result for nums1 using the precomputed map.

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // Stack keeps a decreasing sequence of numbers.
        let mut stack: Vec<i32> = Vec::new();

        // Maps a number to its next greater element.
        let mut next_map: HashMap<i32, i32> = HashMap::new();

        // Traverse nums2 to compute next greater elements.
        for num in nums2 {
            // Resolve next greater for all smaller elements on the stack.
            while let Some(&top) = stack.last() {
                if num > top {
                    stack.pop();
                    next_map.insert(top, num);
                } else {
                    break;
                }
            }
            // Push current number as a candidate.
            stack.push(num);
        }

        // Build result for nums1 using the map.
        let mut result = Vec::new();
        for num in nums1 {
            // If no next greater element exists, return -1.
            result.push(*next_map.get(&num).unwrap_or(&-1));
        }

        result
    }
}