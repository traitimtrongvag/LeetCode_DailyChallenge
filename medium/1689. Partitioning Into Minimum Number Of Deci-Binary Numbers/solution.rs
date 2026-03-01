impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut answer = 0;

        // The result is the maximum digit in the string
        for ch in n.chars() {
            let digit = ch as i32 - '0' as i32;
            if digit > answer {
                answer = digit;
            }
        }

        answer
    }
}