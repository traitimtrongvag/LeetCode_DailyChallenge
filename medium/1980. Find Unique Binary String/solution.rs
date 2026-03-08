impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut res = String::with_capacity(n);

        for (i, s) in nums.iter().enumerate() {
            let c = s.as_bytes()[i];
            if c == b'0' {
                res.push('1');
            } else {
                res.push('0');
            }
        }

        res
    }
}
