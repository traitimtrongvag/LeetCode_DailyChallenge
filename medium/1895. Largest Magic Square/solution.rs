// Algorithm overview:
// Precompute prefix sums for rows, columns, and both diagonals.
// This allows constant-time sum queries for any sub-square.
// For each possible bottom-right corner, try larger squares first.
// A square is magic if all rows, columns, and both diagonals have the same sum.
// Track and return the largest valid square size.

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        // Grid dimensions.
        let (m, n) = (grid.len(), grid[0].len());

        // Prefix sums for rows, columns, and two diagonals.
        let (mut rows, mut cols, mut d1, mut d2) = (
            vec![vec![0; n + 1]; m],     // Row prefix sums.
            vec![vec![0; n]; m + 1],     // Column prefix sums.
            vec![vec![0; n + 1]; m + 1], // Main diagonal prefix sums.
            vec![vec![0; n + 1]; m + 1], // Anti-diagonal prefix sums.
        );

        // Build all prefix sums.
        for i in 0..m {
            for j in 0..n {
                // Row prefix sum.
                rows[i][j + 1] = grid[i][j] + rows[i][j];
                // Column prefix sum.
                cols[i + 1][j] = grid[i][j] + cols[i][j];
                // Main diagonal prefix sum.
                d1[i + 1][j + 1] = grid[i][j] + d1[i][j];
                // Anti-diagonal prefix sum.
                d2[i + 1][n - j - 1] = grid[i][n - j - 1] + d2[i][n - j];
            }
        }

        // Check whether the square defined by (i1, j1) to (i2, j2) is magic.
        let is_magic = |i1: usize, j1: usize, i2: usize, j2: usize, _print: bool| -> bool {
            // Sum of the main diagonal.
            let diag1 = d1[i2 + 1][j2 + 1] - d1[i1][j1];
            // Sum of the anti-diagonal.
            let diag2 = d2[i2 + 1][j1] - d2[i1][j2 + 1];

            // Diagonals must match, and all rows and columns must match that sum.
            diag1 == diag2
                && (i1..=i2).all(|i| (rows[i][j2 + 1] - rows[i][j1]) == diag1)
                && (j1..=j2).all(|j| (cols[i2 + 1][j] - cols[i1][j]) == diag1)
        };

        // The smallest magic square has side length 1.
        let mut ans = 1;

        // Try every cell as the bottom-right corner.
        for i in 0..m {
            for j in 0..n {
                // Try larger squares first to break early when found.
                for side in (2..=1 + i.min(j)).rev() {
                    if is_magic(i - side + 1, j - side + 1, i, j, side == 3) {
                        ans = ans.max(side);
                        break;
                    }
                }
            }
        }

        // Return the largest magic square size.
        ans as _
    }
}