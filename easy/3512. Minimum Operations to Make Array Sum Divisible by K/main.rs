// 3512. Minimum Operations to Make Array Sum Divisible by K
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        (sum % k as i64) as i32
    }
}