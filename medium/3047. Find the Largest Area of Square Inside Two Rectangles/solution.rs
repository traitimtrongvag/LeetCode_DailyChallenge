// Algorithm overview:
// Combine bottom-left and top-right coordinates into rectangle representations.
// Compare every pair of rectangles to find their overlapping region.
// From the overlap, take the largest possible square by using the smaller side.
// Track and return the maximum square area found.

impl Solution {
    pub fn largest_square_area(bl: Vec<Vec<i32>>, tr: Vec<Vec<i32>>) -> i64 {
        // Merge bottom-left and top-right points into a single list of rectangles.
        let mut v: Vec<Vec<i32>> = Vec::new();
        for i in 0..bl.len() {
            // Each rectangle is stored as [x1, y1, x2, y2].
            v.push(vec![bl[i][0], bl[i][1], tr[i][0], tr[i][1]]);
        }

        // Stores the maximum square area found so far.
        let mut ans: i64 = 0;

        // Iterate through all rectangle pairs.
        for i in 0..v.len() {
            let x1 = v[i][0] as i64;
            let y1 = v[i][1] as i64;
            let x2 = v[i][2] as i64;
            let y2 = v[i][3] as i64;

            for j in (i + 1)..v.len() {
                let a1 = v[j][0] as i64;
                let b1 = v[j][1] as i64;
                let a2 = v[j][2] as i64;
                let b2 = v[j][3] as i64;

                // Skip if the two rectangles do not overlap at all.
                if a1 > x2 || a2 < x1 || b1 > y2 || b2 < y1 {
                    continue;
                }

                // Compute the overlapping rectangle boundaries.
                let X1 = x1.max(a1);
                let Y1 = y1.max(b1);
                let X2 = x2.min(a2);
                let Y2 = y2.min(b2);

                // The largest square side is limited by the smaller overlap dimension.
                let mini = (X2 - X1).abs().min((Y2 - Y1).abs());

                // Update the maximum square area.
                ans = ans.max(mini * mini);
            }
        }

        // Return the largest square area found.
        ans
    }
}