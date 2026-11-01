impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = 0;
        let mut col = grid[0].len();
        let mut count = 0;

        while row < grid.len() && col > 0 {
            if grid[row][col - 1] < 0 {
                count += grid.len() - row;
                col -= 1;
            } else {
                row += 1;
            }
        }

        count as i32
    }
}
