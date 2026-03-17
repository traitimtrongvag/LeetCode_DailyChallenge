impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut height = vec![vec![0; cols]; rows];

        for j in 0..cols {
            height[0][j] = matrix[0][j];
        }

        for i in 1..rows {
            for j in 0..cols {
                if matrix[i][j] == 1 {
                    height[i][j] = height[i - 1][j] + 1;
                }
            }
        }

        let mut max_area = 0;

        for i in 0..rows {
            let mut row = height[i].clone();
            row.sort_unstable_by(|a, b| b.cmp(a));

            for j in 0..cols {
                let area = row[j] * (j as i32 + 1);
                max_area = max_area.max(area);
            }
        }

        max_area
    }
}