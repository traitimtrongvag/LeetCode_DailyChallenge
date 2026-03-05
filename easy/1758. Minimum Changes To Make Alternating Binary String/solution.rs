impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut start_with_0 = 0;
        let mut start_with_1 = 0;

        // check mismatches for both patterns: 0101... and 1010...
        for i in 0..chars.len() {
            let expected0 = if i % 2 == 0 { '0' } else { '1' };
            let expected1 = if i % 2 == 0 { '1' } else { '0' };

            if chars[i] != expected0 {
                start_with_0 += 1;
            }

            if chars[i] != expected1 {
                start_with_1 += 1;
            }
        }

        // return the smaller number of changes
        start_with_0.min(start_with_1)
    }
}