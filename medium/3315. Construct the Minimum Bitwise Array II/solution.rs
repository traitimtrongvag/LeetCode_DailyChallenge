// Algorithm overview:
// For each number, analyze its binary representation.
// Count consecutive trailing 1-bits from the least significant side.
// Remove the highest bit within that trailing block to form the result.
// If the value is 2, no valid construction exists, keep -1.
// Store the computed value for each element.

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        // Length of the input array.
        let n = nums.len();

        // Initialize result array with -1.
        let mut a = vec![-1; n];

        // Process each number independently.
        for i in 0..n {
            let mut x = nums[i];

            // Special case: 2 cannot be represented.
            if x == 2 {
                continue;
            }

            let mut y = x;
            let mut c = 0;

            // Count consecutive trailing 1 bits.
            while y & 1 == 1 {
                c += 1;
                y >>= 1;
            }

            // Remove the highest bit in the trailing-ones block.
            x = x - (1 << (c - 1));
            a[i] = x;
        }

        // Return the constructed array.
        a
    }
}