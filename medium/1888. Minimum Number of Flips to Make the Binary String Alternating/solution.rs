impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let s2 = s.clone() + &s; // simulate rotation by doubling the string
        let chars: Vec<char> = s2.chars().collect();

        let mut diff_start_0 = 0;
        let mut diff_start_1 = 0;
        let mut result = i32::MAX;
        let mut left = 0;

        // sliding window of size n
        for right in 0..chars.len() {
            // expected chars for patterns
            let expected0 = if right % 2 == 0 { '0' } else { '1' };
            let expected1 = if right % 2 == 0 { '1' } else { '0' };

            if chars[right] != expected0 {
                diff_start_0 += 1;
            }
            if chars[right] != expected1 {
                diff_start_1 += 1;
            }

            // keep window size = n
            if right - left + 1 > n {
                let expected0 = if left % 2 == 0 { '0' } else { '1' };
                let expected1 = if left % 2 == 0 { '1' } else { '0' };

                if chars[left] != expected0 {
                    diff_start_0 -= 1;
                }
                if chars[left] != expected1 {
                    diff_start_1 -= 1;
                }

                left += 1;
            }

            // when window size == n, update result
            if right - left + 1 == n {
                result = result.min(diff_start_0.min(diff_start_1));
            }
        }

        result
    }
}