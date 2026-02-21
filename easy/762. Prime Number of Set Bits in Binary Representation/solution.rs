impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        // Since max bits for i32 is 32,
        // possible set bits range: 0..=32
        // Predefine prime numbers up to 32
        fn is_prime(n: u32) -> bool {
            match n {
                2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 => true,
                _ => false,
            }
        }

        let mut count = 0;

        for num in left..=right {
            let bits = num.count_ones();
            if is_prime(bits) {
                count += 1;
            }
        }

        count
    }
}