impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let rows = mat.len();
        let cols = mat[0].len();

        let mut row_count = vec![0; rows];
        let mut col_count = vec![0; cols];

        // Count number of 1s in each row and column
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        let mut result = 0;

        // A position is special if:
        // - mat[i][j] == 1
        // - that row has exactly one 1
        // - that column has exactly one 1
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 1 && row_count[i] == 1 && col_count[j] == 1 {
                    result += 1;
                }
            }
        }

        result
    }
}