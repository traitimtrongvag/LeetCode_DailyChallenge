impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        let mut result: i32 = 0;

        for _ in 0..32 {
            result <<= 1;          // Make space for next bit
            result |= n & 1;       // Add lowest bit of n
            n >>= 1;               // Shift n to process next bit
        }

        result
    }
}