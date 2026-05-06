/*
Time: O(n)
Space: O(n)

Idea: filter + lowercase, then check palindrome
*/

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        v.iter().eq(v.iter().rev())
    }
}