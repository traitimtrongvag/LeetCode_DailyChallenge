// Algorithm overview:
// Build a 2D prefix sum to query sub-square sums in constant time.
// Use binary search on the side length of the square.
// For each candidate size, check if any square has sum <= threshold.
// If possible, try a larger size; otherwise, reduce the size.
// Return the maximum valid side length.

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        // Matrix dimensions.
        let m = mat.len();
        let n = mat[0].len();

        // 2D prefix sum array.
        let mut prefix = vec![vec![0; n + 1]; m + 1];

        // Build prefix sums.
        for i in 0..m {
            for j in 0..n {
                prefix[i + 1][j + 1] =
                    prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j] + mat[i][j];
            }
        }

        // Binary search bounds for square side length.
        let mut left = 0;
        let mut right = std::cmp::min(m, n) as i32;
        let mut answer = 0;

        // Binary search for the largest valid square.
        while left <= right {
            let mid = (left + right) / 2;
            let size = mid as usize;
            let mut found = false;

            // Check all possible squares of current size.
            for i in size..=m {
                for j in size..=n {
                    let sum = prefix[i][j]
                        - prefix[i - size][j]
                        - prefix[i][j - size]
                        + prefix[i - size][j - size];
                    if sum <= threshold {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            // Adjust search range based on feasibility.
            if found {
                answer = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // Return the maximum side length found.
        answer
    }
}