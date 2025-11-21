// 1930. Unique Length-3 Palindromic Subsequences
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();

        let mut first = vec![-1; 26];
        let mut last = vec![-1; 26];

        for (i, &c) in bytes.iter().enumerate() {
            let idx = (c - b'a') as usize;
            if first[idx] == -1 {
                first[idx] = i as i32;
            }
            last[idx] = i as i32;
        }

        let mut ans = 0;

        for a in 0..26 {
            let l = first[a];
            let r = last[a];
            if l != -1 && r - l > 1 {
                let mut seen = [false; 26];
                for i in (l + 1) as usize..r as usize {
                    let b = (bytes[i] - b'a') as usize;
                    if !seen[b] {
                        seen[b] = true;
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}