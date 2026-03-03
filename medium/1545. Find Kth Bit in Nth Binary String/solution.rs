impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        // Base case:
        // S1 = "0"
        if n == 1 {
            return '0';
        }

        // Length of Sn = 2^n - 1
        let length = (1 << n) - 1;
        let mid = length / 2 + 1; // middle position (1-based index)

        if k == mid {
            // The middle character is always '1'
            return '1';
        } else if k < mid {
            // Left half is exactly Sn-1
            return Self::find_kth_bit(n - 1, k);
        } else {
            // Right half is reverse(invert(Sn-1))
            // Mirror position in left half
            let mirrored = length - k + 1;

            let bit = Self::find_kth_bit(n - 1, mirrored);

            // Invert the bit
            if bit == '0' { '1' } else { '0' }
        }
    }
}