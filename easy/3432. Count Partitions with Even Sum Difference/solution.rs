impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left = 0;
        let mut ans = 0;
        for i in 0..nums.len() - 1 {
            left += nums[i];
            let right = total - left;
            if ((left - right) & 1) == 0 {
                ans += 1;
            }
        }
        ans
    }
}