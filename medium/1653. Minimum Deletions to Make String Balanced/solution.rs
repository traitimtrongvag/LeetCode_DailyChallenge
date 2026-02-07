impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut stack = Vec::<char>::new();
        // number of deletions performed
        let mut count = 0;

        let char_vec: Vec<char> = s.chars().collect();

        for ch in &char_vec {
            // if we see "ba", delete the 'b'
            if !stack.is_empty() && *stack.last().unwrap() == 'b' && *ch == 'a' {
                stack.pop();
                count += 1;
            } else {
                // otherwise keep the character
                stack.push(*ch);
            }
        }

        count
    }
}