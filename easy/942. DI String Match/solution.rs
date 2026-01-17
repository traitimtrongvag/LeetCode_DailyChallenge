// Algorithm overview:
// Use a greedy two-pointer approach.
// Maintain the smallest and largest available numbers.
// For 'I', place the smallest value to keep the sequence increasing.
// For 'D', place the largest value to keep the sequence decreasing.
// After processing all characters, one number remains and is appended at the end.

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        // Result permutation.
        let mut n: Vec<i32> = vec![];

        // Smallest available number.
        let mut i = 0;
        // Largest available number.
        let mut d: i32 = s.len() as i32;

        // Iterate through each character in the string.
        for (j, v) in s.chars().enumerate() {
            // For 'I', assign the current smallest value.
            if v == 'I' {
                n.push(i);
                i += 1;
            }
            // For 'D', assign the current largest value.
            else if v == 'D' {
                n.push(d);
                d -= 1;
            }

            // After the last character, append the remaining value.
            if j == s.len() - 1 {
                if v == 'I' {
                    n.push(i);
                    i += 1;
                } else if v == 'D' {
                    n.push(d);
                    d -= 1;
                }
            }
        }

        // Return the constructed permutation.
        n
    }
}