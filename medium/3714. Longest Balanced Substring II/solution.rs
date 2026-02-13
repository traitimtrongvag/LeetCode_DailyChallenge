use std::collections::HashMap;

impl Solution {
    // Longest substring with only one repeated character
    fn longest_single_char(s: &str) -> i32 {
        let mut max_len = 1;
        let mut prev_char = '\0';
        let mut current_len = 0;

        for ch in s.chars() {
            if ch == prev_char {
                current_len += 1;
            } else {
                current_len = 1;
                prev_char = ch;
            }
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }

    // Longest substring with exactly two characters balanced,
    // ignoring one specific character
    fn longest_two_balanced(s: &str, excluded: char) -> i32 {
        let mut first_seen: HashMap<i32, i32> = HashMap::new();
        first_seen.insert(0, -1);

        let mut delta = 0;
        let mut max_len = 0;

        let chars: Vec<char> = ['a', 'b', 'c']
            .into_iter()
            .filter(|&c| c != excluded)
            .collect();

        let c1 = chars[0];
        let c2 = chars[1];

        for (idx, ch) in s.chars().enumerate() {
            let idx = idx as i32;

            if ch == excluded {
                first_seen.clear();
                first_seen.insert(0, idx);
                delta = 0;
                continue;
            }

            if ch == c1 {
                delta += 1;
            } else if ch == c2 {
                delta -= 1;
            }

            if let Some(&start) = first_seen.get(&delta) {
                let length = idx - start;
                if length > max_len {
                    max_len = length;
                }
            } else {
                first_seen.insert(delta, idx);
            }
        }

        max_len
    }

    // Longest substring where count(a) == count(b) == count(c)
    fn longest_three_balanced(s: &str) -> i32 {
        let mut first_seen: HashMap<(i32, i32), i32> = HashMap::new();
        first_seen.insert((0, 0), -1);

        let mut count_a = 0;
        let mut count_b = 0;
        let mut count_c = 0;
        let mut max_len = 0;

        for (idx, ch) in s.chars().enumerate() {
            let idx = idx as i32;

            match ch {
                'a' => count_a += 1,
                'b' => count_b += 1,
                'c' => count_c += 1,
                _ => {}
            }

            // Use differences to represent balanced state
            let diff_ba = count_b - count_a;
            let diff_ca = count_c - count_a;

            if let Some(&start) = first_seen.get(&(diff_ba, diff_ca)) {
                let length = idx - start;
                if length > max_len {
                    max_len = length;
                }
            } else {
                first_seen.insert((diff_ba, diff_ca), idx);
            }
        }

        max_len
    }

    pub fn longest_balanced(s: String) -> i32 {
        let mut result = Self::longest_single_char(&s);

        for excluded in ['a', 'b', 'c'] {
            let len = Self::longest_two_balanced(&s, excluded);
            if len > result {
                result = len;
            }
        }

        let len = Self::longest_three_balanced(&s);
        if len > result {
            result = len;
        }

        result
    }
}