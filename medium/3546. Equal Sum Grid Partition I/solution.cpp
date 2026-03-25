impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut total_sum: i64 = 0;

        // get total sum of grid
        for row in &grid {
            for &val in row {
                total_sum += val as i64;
            }
        }

        // odd total -> can't split into 2 equal parts
        if total_sum % 2 != 0 {
            return false;
        }

        let target = total_sum / 2;

        // try cut by rows
        let mut row_sum: i64 = 0;
        for i in 0..rows {
            let mut cur = 0i64;

            // sum current row
            for j in 0..cols {
                cur += grid[i][j] as i64;
            }

            row_sum += cur;

            if row_sum == target {
                return true; // found valid split
            }

            if row_sum > target {
                break; // no point continue
            }
        }

        // try cut by columns
        let mut col_sum: i64 = 0;
        for j in 0..cols {
            let mut cur = 0i64;

            // sum current column
            for i in 0..rows {
                cur += grid[i][j] as i64;
            }

            col_sum += cur;

            if col_sum == target {
                return true;
            }

            if col_sum > target {
                break;
            }
        }

        false
    }
}