use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // Quick length check: if lengths differ, cannot be anagrams
        if s.len() != t.len() {
            return false;
        }

        let mut counter: HashMap<char, i32> = HashMap::new();

        // Count frequency of each char in `s`
        for ch in s.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        // Subtract frequency for each char in `t`
        for ch in t.chars() {
            if let Some(count) = counter.get_mut(&ch) {
                *count -= 1;
                // If count goes negative, `t` has extra chars not in `s`
                if *count < 0 {
                    return false;
                }
            } else {
                // Char in `t` not in `s` → not an anagram
                return false;
            }
        }

        // All counts matched → valid anagram
        true
    }
}