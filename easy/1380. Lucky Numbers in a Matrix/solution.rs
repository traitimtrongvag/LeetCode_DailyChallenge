impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut row_min = vec![i32::MAX; rows];
        let mut col_max = vec![i32::MIN; cols];

        // Find minimum value in each row
        for i in 0..rows {
            for j in 0..cols {
                row_min[i] = row_min[i].min(matrix[i][j]);
            }
        }

        // Find maximum value in each column
        for j in 0..cols {
            for i in 0..rows {
                col_max[j] = col_max[j].max(matrix[i][j]);
            }
        }

        let mut result = Vec::new();

        // A lucky number is both row min and column max
        for i in 0..rows {
            for j in 0..cols {
                let value = matrix[i][j];
                if value == row_min[i] && value == col_max[j] {
                    result.push(value);
                }
            }
        }

        result
    }
}