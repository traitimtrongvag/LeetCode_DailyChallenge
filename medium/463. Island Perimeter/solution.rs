// Algorithm overview:
// Traverse every cell in the grid.
// For each land cell, check its four neighboring directions.
// If a neighbor is out of bounds or water, that side contributes to the perimeter.
// Sum all such exposed sides to get the final perimeter.

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        // Grid dimensions.
        let (m, n) = (grid.len(), grid[0].len());

        // Four possible movement directions: right, down, left, up.
        let direction = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Accumulates the total perimeter.
        let mut res = 0;

        // Iterate through each cell in the grid.
        for x in 0..m {
            for y in 0..n {
                // Skip water cells.
                if grid[x][y] == 0 {
                    continue;
                }

                // Check all four neighbors of the current land cell.
                for &(dx, dy) in direction.iter() {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);

                    // If neighbor is outside the grid or is water,
                    // this side contributes 1 to the perimeter.
                    if nx < 0
                        || nx == m as i32
                        || ny < 0
                        || ny == n as i32
                        || grid[nx as usize][ny as usize] == 0
                    {
                        res += 1;
                    }
                }
            }
        }

        // Return the total perimeter.
        res
    }
}