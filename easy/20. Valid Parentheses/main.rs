// 20. Valid Parentheses
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => if stack.pop() != Some('(') { return false },
                ']' => if stack.pop() != Some('[') { return false },
                '}' => if stack.pop() != Some('{') { return false },
                _ => return false
            }
        }
        stack.is_empty()
    }
}