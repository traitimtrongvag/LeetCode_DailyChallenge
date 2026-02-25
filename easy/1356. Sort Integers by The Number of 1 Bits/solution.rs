impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut nums = arr;

        nums.sort_by(|a, b| {
            let bits_a = a.count_ones();
            let bits_b = b.count_ones();

            // First sort by number of 1-bits
            // If equal, sort by numeric value
            bits_a.cmp(&bits_b).then(a.cmp(b))
        });

        nums
    }
}