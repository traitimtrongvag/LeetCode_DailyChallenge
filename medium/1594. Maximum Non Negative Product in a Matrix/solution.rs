impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        // dp_min[i][j] = min product to reach (i,j)
        // dp_max[i][j] = max product to reach (i,j)
        let mut dp_min = vec![vec![0i128; cols]; rows];
        let mut dp_max = vec![vec![0i128; cols]; rows];

        dp_min[0][0] = grid[0][0] as i128;
        dp_max[0][0] = grid[0][0] as i128;

        // first row
        for j in 1..cols {
            let val = grid[0][j] as i128;
            dp_min[0][j] = dp_min[0][j - 1] * val;
            dp_max[0][j] = dp_max[0][j - 1] * val;
        }

        // first column
        for i in 1..rows {
            let val = grid[i][0] as i128;
            dp_min[i][0] = dp_min[i - 1][0] * val;
            dp_max[i][0] = dp_max[i - 1][0] * val;
        }

        for i in 1..rows {
            for j in 1..cols {
                let val = grid[i][j] as i128;

                // from top
                let top_min = dp_min[i - 1][j] * val;
                let top_max = dp_max[i - 1][j] * val;

                // from left
                let left_min = dp_min[i][j - 1] * val;
                let left_max = dp_max[i][j - 1] * val;

                // pick min/max carefully (negative flips sign)
                dp_min[i][j] = *[top_min, top_max, left_min, left_max]
                    .iter()
                    .min()
                    .unwrap();

                dp_max[i][j] = *[top_min, top_max, left_min, left_max]
                    .iter()
                    .max()
                    .unwrap();
            }
        }

        let result = dp_max[rows - 1][cols - 1];

        if result < 0 {
            return -1;
        }

        // mod as required
        (result % 1_000_000_007) as i32
    }
}