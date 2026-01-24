// Algorithm overview:
// Sort the array to pair smallest and largest values together.
// Use two pointers from both ends of the sorted array.
// For each pair, compute the sum and track the maximum among them.
// This pairing minimizes the maximum pair sum.
// Return the largest pair sum found.

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        // Sort the numbers in ascending order.
        let mut nums = nums;
        nums.sort();

        // Two pointers for pairing.
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        // Stores the maximum pair sum.
        let mut answer = 0;

        // Pair smallest with largest until pointers meet.
        while left < right {
            let sum = nums[left] + nums[right];

            // Update the maximum pair sum.
            if sum > answer {
                answer = sum;
            }

            left += 1;
            right -= 1;
        }

        // Return the minimized maximum pair sum.
        answer
    }
}