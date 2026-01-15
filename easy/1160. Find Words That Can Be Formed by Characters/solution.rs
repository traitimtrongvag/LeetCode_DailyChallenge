impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        // Count frequency of each character in 'chars' string
        // Mutable array tracking counts for a-z (index 0-25)
        let mut bank = [0; 26];
        for i in chars.as_bytes() {
            bank[*i as usize - 'a' as usize] += 1;  // Increment count for character
        }
        
        // Make 'bank' immutable for the base character frequency reference
        let bank = bank;    // make immutable
        
        let mut ans = 0;
        
        // Process each word in the input words list
        'out: for w in words.iter() {
            // Create a mutable copy of character bank for this word
            let mut bank = bank;
            
            // Check if current word can be formed from 'chars' characters
            for i in w.as_bytes() {
                let idx = *i as usize - 'a' as usize;  // Calculate index 0-25
                bank[idx] -= 1;  // Use one occurrence of this character
                
                // If we used more of a character than available, word is invalid
                if bank[idx] < 0 {
                    continue 'out;  // Skip to next word
                }
            }
            
            // Word can be formed - add its length to answer
            ans += w.len();
        }
        
        // Return total length of all formable words
        ans as i32
    }
}