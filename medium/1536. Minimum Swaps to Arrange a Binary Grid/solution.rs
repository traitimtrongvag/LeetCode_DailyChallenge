impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut trailing_zeros = Vec::with_capacity(n);

        // Count trailing zeros in each row
        for row in grid.iter() {
            let mut count = 0;
            for &v in row.iter().rev() {
                if v == 0 {
                    count += 1;
                } else {
                    break;
                }
            }
            trailing_zeros.push(count);
        }

        let mut swaps = 0;

        // For each row position, we need at least (n - 1 - i) trailing zeros
        for i in 0..n {
            let needed = n - 1 - i;
            let mut j = i;

            // Find the first row below that satisfies the condition
            while j < n && trailing_zeros[j] < needed {
                j += 1;
            }

            // If not found, impossible
            if j == n {
                return -1;
            }

            // Bring row j up to position i by swapping adjacent rows
            while j > i {
                trailing_zeros.swap(j, j - 1);
                swaps += 1;
                j -= 1;
            }
        }

        swaps
    }
}