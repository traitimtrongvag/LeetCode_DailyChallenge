impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x1 = points[0][0];
        let y1 = points[0][1];

        let x2 = points[1][0];
        let y2 = points[1][1];

        let x3 = points[2][0];
        let y3 = points[2][1];

        // Compute twice the triangle area using cross product formula
        // If result is 0, three points are collinear
        let area2 =
            x1 * (y2 - y3) +
            x2 * (y3 - y1) +
            x3 * (y1 - y2);

        area2 != 0
    }
}