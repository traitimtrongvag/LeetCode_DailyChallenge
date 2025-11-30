// 125. Valid Palindrome
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        filtered.iter().eq(filtered.iter().rev())
    }
}