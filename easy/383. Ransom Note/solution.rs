use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Count characters in magazine
        let mut count: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        // Try to use characters from magazine
        for c in ransom_note.chars() {
            if count.get(&c).unwrap_or(&0) <= &0 {
                return false;
            }
            *count.get_mut(&c).unwrap() -= 1;
        }

        true
    }
}