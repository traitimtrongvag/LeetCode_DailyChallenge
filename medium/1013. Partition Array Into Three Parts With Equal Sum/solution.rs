// Algorithm overview:
// Compute the total sum of the array.
// If the sum is not divisible by 3, it is impossible to split.
// Otherwise, scan the array and accumulate values.
// Each time the running sum reaches target, form one partition.
// If at least three partitions are found, return true.

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        // Calculate the total sum of the array.
        let total: i32 = arr.iter().sum();

        // If total is not divisible by 3, split is impossible.
        if total % 3 != 0 {
            return false;
        }

        // Each part must sum to this value.
        let target = total / 3;

        // Current running sum for the ongoing partition.
        let mut current_sum = 0;
        // Number of valid partitions found.
        let mut count = 0;

        // Traverse the array and try to form partitions.
        for num in arr {
            current_sum += num;

            // When a partition reaches the target sum,
            // reset and count it as one valid part.
            if current_sum == target {
                count += 1;
                current_sum = 0;
            }
        }

        // At least three parts are required.
        count >= 3
    }
}