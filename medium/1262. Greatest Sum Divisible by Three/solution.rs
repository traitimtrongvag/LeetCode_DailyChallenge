impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, i32::MIN, i32::MIN];
        
        for num in nums {
            let temp = dp.clone();
            for i in 0..3 {
                if temp[i] != i32::MIN {
                    let sum = temp[i] + num;
                    let remainder = (sum % 3) as usize;
                    dp[remainder] = dp[remainder].max(sum);
                }
            }
        }
        
        dp[0]
    }
}