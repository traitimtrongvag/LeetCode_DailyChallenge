impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut ans = 0;
        let mut pref = vec![vec![(0, 0); n]; m]; // (countX, countY)

        for i in 0..m {
            for j in 0..n {
                let mut x = 0;
                let mut y = 0;

                if grid[i][j] == 'X' { x = 1; }
                if grid[i][j] == 'Y' { y = 1; }

                if i > 0 {
                    x += pref[i - 1][j].0;
                    y += pref[i - 1][j].1;
                }
                if j > 0 {
                    x += pref[i][j - 1].0;
                    y += pref[i][j - 1].1;
                }
                if i > 0 && j > 0 {
                    x -= pref[i - 1][j - 1].0;
                    y -= pref[i - 1][j - 1].1;
                }

                pref[i][j] = (x, y);

                if x > 0 && x == y {
                    ans += 1;
                }
            }
        }

        ans
    }
}