use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_index = HashMap::new();
        let mut max_length = 0;
        let mut left = 0;
        let chars: Vec<char> = s.chars().collect();
        
        for right in 0..chars.len() {
            let current_char = chars[right];
            
            if let Some(&index) = char_index.get(&current_char) {
                if index >= left {
                    left = index + 1;
                }
            }
            
            char_index.insert(current_char, right);
            max_length = max(max_length, right - left + 1);
        }
        
        max_length as i32
    }
}