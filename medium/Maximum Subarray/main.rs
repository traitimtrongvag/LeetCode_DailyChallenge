// 53. Maximum Subarray
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum = 0;
        let mut max_sum = nums[0];

        for &x in &nums {
            current_sum = (current_sum + x).max(x);
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}