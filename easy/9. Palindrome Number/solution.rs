impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut x = x;
        let mut reversed = 0;
        while x > reversed {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }
        x == reversed || x == reversed / 10
    }
}