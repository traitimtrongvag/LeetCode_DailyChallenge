// Algorithm overview:
// Generate all cell coordinates in the grid.
// Sort the cells by their Manhattan distance to the given center.
// Manhattan distance is computed as |r - r_center| + |c - c_center|.
// Return the sorted list.

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        // Store all cell coordinates.
        let mut cells = Vec::new();

        // Enumerate every cell in the grid.
        for r in 0..rows {
            for c in 0..cols {
                cells.push(vec![r, c]);
            }
        }

        // Sort cells by Manhattan distance to the center.
        cells.sort_by_key(|cell| {
            (cell[0] - r_center).abs() + (cell[1] - c_center).abs()
        });

        // Return cells ordered by distance.
        cells
    }
}
