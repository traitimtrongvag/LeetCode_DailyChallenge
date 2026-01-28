impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut g = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        g[0][0] = 0;
        let downright_propagation = |g: &mut [Vec<i32>]| {
            let mut grid_max = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    grid_max = grid_max.max(grid[i][j]);
                    if i > 0 && j > 0 {
                        g[i][j] = g[i][j].min(g[i-1][j].min(g[i][j-1]) + grid[i][j]);
                    } else if i > 0 {
                        g[i][j] = g[i][j].min(g[i-1][j] + grid[i][j]);
                    } else if j > 0 {
                        g[i][j] = g[i][j].min(g[i][j-1] + grid[i][j]);
                    }
                }
            }
            grid_max
        };
        let grid_max = downright_propagation(&mut g) as usize;
        for _ in 0..k {
            let mut min_steps_for_level = vec![i32::MAX; grid_max + 1];
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    min_steps_for_level[grid[i][j] as usize] = min_steps_for_level[grid[i][j] as usize].min(g[i][j]);
                }
            }
            for i in (0..grid_max).rev() {
                min_steps_for_level[i] = min_steps_for_level[i].min(min_steps_for_level[i+1]);
            }
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    g[i][j] = g[i][j].min(min_steps_for_level[grid[i][j] as usize]);
                }
            }
            downright_propagation(&mut g);
        }
        *g.last().unwrap().last().unwrap()
    }
}