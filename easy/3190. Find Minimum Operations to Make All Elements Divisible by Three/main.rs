// 3190. Find Minimum Operations to Make All Elements Divisible by Three
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        for &num in &nums {
            let remainder = num % 3;
            operations += remainder.min(3 - remainder);
        }
        operations
    }
}