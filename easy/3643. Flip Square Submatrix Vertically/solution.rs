impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        // loop each column inside k x k block
        for i in 0..k {
            // swap vertically (top <-> bottom)
            for j in 0..k / 2 {
                let top = x + j;
                let bottom = x + k - j - 1;

                let temp = grid[top][y + i];
                grid[top][y + i] = grid[bottom][y + i];
                grid[bottom][y + i] = temp;
            }
        }

        grid
    }
}