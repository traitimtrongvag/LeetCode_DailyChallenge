// dp[j][k] = max sum at column j using k skips
// k = 0..2 (can skip up to 2 negative cells)

impl Solution {
    pub fn maximum_amount(a: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (a.len(), a[0].len());
        let neg = i32::MIN / 2;

        let mut dp = vec![[neg; 3]; n];

        // init first cell
        for k in 0..3 {
            dp[0][k] = if k > 0 { a[0][0].max(0) } else { a[0][0] };
        }

        // first row
        for j in 1..n {
            for k in (0..3).rev() {
                dp[j][k] = dp[j - 1][k] + a[0][j];
                if k > 0 {
                    dp[j][k] = dp[j][k].max(dp[j - 1][k - 1]); // skip
                }
            }
        }

        // rest rows
        for i in 1..m {
            let mut next = vec![[neg; 3]; n];

            for j in 0..n {
                for k in (0..3).rev() {
                    // from top
                    if dp[j][k] != neg {
                        next[j][k] = dp[j][k] + a[i][j];
                    }
                    if k > 0 && dp[j][k - 1] != neg {
                        next[j][k] = next[j][k].max(dp[j][k - 1]); // skip
                    }

                    // from left
                    if j > 0 {
                        if next[j - 1][k] != neg {
                            next[j][k] = next[j][k].max(next[j - 1][k] + a[i][j]);
                        }
                        if k > 0 && next[j - 1][k - 1] != neg {
                            next[j][k] = next[j][k].max(next[j - 1][k - 1]); // skip
                        }
                    }
                }
            }

            dp = next;
        }

        dp[n - 1][2]
    }
}