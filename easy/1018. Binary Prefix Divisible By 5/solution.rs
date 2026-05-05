/*
Time: O(n)
Space: O(1) (excluding output)

Idea: build binary prefix mod 5, only keep remainder
*/

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut res = Vec::with_capacity(nums.len());
        let mut cur = 0; // current value mod 5
        
        for &bit in &nums {
            // shift left (x2) and add new bit, keep mod 5
            cur = (cur * 2 + bit) % 5;

            // divisible by 5 if remainder == 0
            res.push(cur == 0);
        }
        
        res
    }
}