impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let modulo: i64 = 1_000_000_007;
        
        let mut dp = vec![0i64; n + 1];
        dp[0] = 1;

        let mut mx = vec![0usize; n];
        let mut mn = vec![0usize; n];
        let mut mxl = 0usize;
        let mut mxr = 0usize;
        let mut mnl = 0usize;
        let mut mnr = 0usize;

        let mut l = 0usize;
        let mut sum: i64 = 0;

        for r in 0..n {
            while mxl < mxr && nums[mx[mxr - 1]] <= nums[r] {
                mxr -= 1;
            }
            while mnl < mnr && nums[mn[mnr - 1]] >= nums[r] {
                mnr -= 1;
            }
            mx[mxr] = r;
            mxr += 1;
            mn[mnr] = r;
            mnr += 1;

            while nums[mx[mxl]] - nums[mn[mnl]] > k {
                if mx[mxl] == l { mxl += 1; }
                if mn[mnl] == l { mnl += 1; }
                sum = (sum - dp[l] + modulo) % modulo;
                l += 1;
            }

            sum = (sum + dp[r]) % modulo;
            dp[r + 1] = sum;
        }

        dp[n] as i32
    }
}