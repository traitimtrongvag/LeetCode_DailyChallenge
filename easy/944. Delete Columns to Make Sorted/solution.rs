impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let rows = strs.len();
        let matrix: Vec<Vec<u8>> = strs.iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();

        let cols = matrix[0].len();
        let mut count = 0;

        for col in 0..cols {
            if !Self::is_sorted(&matrix, col) {
                count += 1;
            }
        }

        count
    }

    // Check if a column is non-decreasing from top to bottom
    fn is_sorted(matrix: &Vec<Vec<u8>>, col: usize) -> bool {
        for row in 0..matrix.len() - 1 {
            if matrix[row][col] > matrix[row + 1][col] {
                return false;
            }
        }
        true
    }
}