// Algorithm overview:
// If k is 1 or less, the difference is always zero.
// Sort the array to bring close values together.
// Use a sliding window of size k over the sorted array.
// For each window, compute the difference between max and min.
// Track and return the minimum difference found.

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        // A single element always has zero difference.
        if k <= 1 {
            return 0;
        }

        // Sort the array to compare adjacent ranges.
        let mut nums = nums;
        nums.sort();

        // Convert k to usize for indexing.
        let k = k as usize;

        // Initialize answer with a large value.
        let mut answer = i32::MAX;

        // Slide a window of size k across the array.
        for i in 0..=nums.len() - k {
            // Difference between the largest and smallest in the window.
            let diff = nums[i + k - 1] - nums[i];

            // Update the minimum difference.
            if diff < answer {
                answer = diff;
            }
        }

        // Return the smallest difference found.
        answer
    }
}