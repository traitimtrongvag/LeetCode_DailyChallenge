use std::collections::BTreeSet;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut set = BTreeSet::new();

        for i in 0..m {
            for j in 0..n {
                // size = 0 (single cell)
                set.insert(grid[i][j]);

                // k = side length of rhombus
                for k in 1.. {
                    // check if rhombus fits inside grid
                    if i + 2 * k >= m || j < k || j + k >= n {
                        break;
                    }

                    let mut sum = 0;

                    // traverse 4 edges of rhombus (avoid double counting corners)

                    // top -> right
                    for t in 0..k {
                        sum += grid[i + t][j + t];
                    }

                    // right -> bottom
                    for t in 0..k {
                        sum += grid[i + k + t][j + k - t];
                    }

                    // bottom -> left
                    for t in 0..k {
                        sum += grid[i + 2 * k - t][j - t];
                    }

                    // left -> top
                    for t in 0..k {
                        sum += grid[i + k - t][j - k + t];
                    }

                    set.insert(sum);
                }
            }
        }

        // take top 3 largest unique sums
        set.into_iter().rev().take(3).collect()
    }
}