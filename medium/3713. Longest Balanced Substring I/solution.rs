impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut max_len: usize = 0;

        for start in 0..n {
            let mut freq = [0usize; 26];

            for end in start..n {
                let index = (bytes[end] - b'a') as usize;
                freq[index] += 1;

                // Check if all non-zero frequencies are equal
                let mut target = 0usize;
                let mut valid = true;

                for &count in freq.iter() {
                    if count == 0 {
                        continue;
                    }

                    if target == 0 {
                        target = count;
                    } else if count != target {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let length = end - start + 1;
                    if length > max_len {
                        max_len = length;
                    }
                }
            }
        }

        max_len as i32
    }
}