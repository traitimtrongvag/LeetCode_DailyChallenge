impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut current = 0;
        
        for &num in &nums {
            current = (current * 2 + num) % 5;
            result.push(current == 0);
        }
        
        result
    }
}