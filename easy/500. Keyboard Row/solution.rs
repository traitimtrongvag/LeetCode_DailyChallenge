use std::collections::HashSet;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let row1: HashSet<char> = "qwertyuiop".chars().collect();
        let row2: HashSet<char> = "asdfghjkl".chars().collect();
        let row3: HashSet<char> = "zxcvbnm".chars().collect();

        let mut result = Vec::new();

        for word in words {
            let lower = word.to_lowercase();
            let mut r1 = true;
            let mut r2 = true;
            let mut r3 = true;

            for ch in lower.chars() {
                if !row1.contains(&ch) {
                    r1 = false;
                }
                if !row2.contains(&ch) {
                    r2 = false;
                }
                if !row3.contains(&ch) {
                    r3 = false;
                }
            }

            if r1 || r2 || r3 {
                result.push(word);
            }
        }

        result
    }
}