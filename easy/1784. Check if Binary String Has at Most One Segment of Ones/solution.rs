impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut seen_zero = false;

        for &b in bytes {
            if b == b'0' {
                // once we see a zero, any later '1' would start a new segment
                seen_zero = true;
            } else if seen_zero {
                // found '1' after a zero → means another segment of ones
                return false;
            }
        }

        true
    }
}