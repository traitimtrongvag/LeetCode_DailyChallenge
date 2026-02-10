// Brute force
use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut best = 0;

        // Try every possible starting index of the subarray
        for start in 0..n {
            // Map to track frequency of each value in current window
            let mut freq: HashMap<i32, i32> = HashMap::new();
            let mut even_count = 0;
            let mut odd_count = 0;

            // Expand the subarray to the right
            for end in start..n {
                // If first time seeing this number, classify it as even/odd
                if !freq.contains_key(&nums[end]) {
                    if nums[end] % 2 == 0 {
                        even_count += 1;
                    } else {
                        odd_count += 1;
                    }
                }

                // Increase its frequency
                *freq.entry(nums[end]).or_insert(0) += 1;

                // Check balance condition: same number of distinct even and odd values
                if even_count == odd_count && even_count > 0 {
                    let length = (end - start + 1) as i32;
                    if length > best {
                        best = length;
                    }
                }
            }
        }

        best
    }
}