// Algorithm overview:
// For each number x, try to find the smallest integer i
// such that i | (i + 1) equals x.
// Even numbers can never satisfy this condition, return -1 immediately.
// For odd numbers, brute-force i from 1 up to x - 1.
// Collect the result for each element.

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&x| {
                // Even values cannot be formed by i | (i + 1).
                if x % 2 == 0 {
                    return -1;
                }

                // Try all possible i values.
                for i in 1..x {
                    // Check if bitwise OR matches x.
                    if (i | (i + 1)) == x {
                        return i;
                    }
                }

                // No valid i found.
                -1
            })
            .collect()
    }
}
