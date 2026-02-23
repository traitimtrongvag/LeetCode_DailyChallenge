use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = s.len();

        // If total possible substrings is less than 2^k, impossible
        if n < k || (n - k + 1) < (1 << k) {
            return false;
        }

        let mut seen = HashSet::new();
        let bytes = s.as_bytes();

        // Sliding window of size k
        for i in 0..=n - k {
            seen.insert(&bytes[i..i + k]);
        }

        // Check if we collected all possible binary codes
        seen.len() == (1 << k)
    }
}