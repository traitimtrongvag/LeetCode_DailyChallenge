impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut x = n;

        // Compare each bit with the next one
        while x > 0 {
            let last_bit = x & 1;          // current bit
            let next_bit = (x >> 1) & 1;   // next bit

            if last_bit == next_bit {
                return false;
            }

            x >>= 1; // shift to check next pair
        }

        true
    }
}