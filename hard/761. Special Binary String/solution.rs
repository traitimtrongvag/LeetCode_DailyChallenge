impl Solution {
    pub fn make_largest_special(s: String) -> String {
        fn helper(s: &str) -> String {
            let mut count = 0;
            let mut start = 0;
            let mut parts: Vec<String> = Vec::new();

            for (i, ch) in s.char_indices() {
                if ch == '1' {
                    count += 1;
                } else {
                    count -= 1;
                }

                // When count == 0, we found a special substring
                if count == 0 {
                    // Recursively process inside substring
                    let inner = helper(&s[start + 1..i]);
                    parts.push(format!("1{}0", inner));
                    start = i + 1;
                }
            }

            // Sort descending to make lexicographically largest
            parts.sort_by(|a, b| b.cmp(a));

            parts.concat()
        }

        helper(&s)
    }
}