impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        
        let m = haystack.len();
        let n = needle.len();
        
        if m < n {
            return -1;
        }
        
        for (i, window) in haystack.as_bytes().windows(n).enumerate() {
            if window == needle.as_bytes() {
                return i as i32;
            }
        }
        
        -1
    }
}