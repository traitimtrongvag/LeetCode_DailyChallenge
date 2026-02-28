impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let mut result: i64 = 0;

        for i in 1..=n as i64 {
            // number of bits needed to represent i
            let bits = 64 - i.leading_zeros() as i64;

            // shift left to make space, then add i
            result = ((result << bits) + i) % modulo;
        }

        result as i32
    }
}