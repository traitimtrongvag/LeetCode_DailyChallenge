impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        fn solve(s: &[u8]) -> Vec<u8> {
            if s.len() < 2 {
                return vec![];
            }

            let mut set = [false; 128];
            for &c in s {
                set[c as usize] = true;
            }

            for i in 0..s.len() {
                let c = s[i];
                if set[(c as char).to_ascii_lowercase() as usize]
                    && set[(c as char).to_ascii_uppercase() as usize]
                {
                    continue;
                }

                let left = solve(&s[..i]);
                let right = solve(&s[i + 1..]);

                return if left.len() >= right.len() { left } else { right };
            }

            s.to_vec()
        }

        String::from_utf8(solve(s.as_bytes())).unwrap()
    }
}