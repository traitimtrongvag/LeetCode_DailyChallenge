impl Solution {
    pub fn champagne_tower(
        poured: i32,
        query_row: i32,
        query_glass: i32,
    ) -> f64 {
        let mut current_row = vec![poured as f64];

        for row_index in 0..query_row as usize {
            let mut next_row = vec![0.0; row_index + 2];

            for glass_index in 0..=row_index {
                let amount = current_row[glass_index];

                if amount > 1.0 {
                    let overflow = (amount - 1.0) / 2.0;
                    next_row[glass_index] += overflow;
                    next_row[glass_index + 1] += overflow;
                }
            }

            current_row = next_row;
        }

        current_row[query_glass as usize].min(1.0)
    }
}