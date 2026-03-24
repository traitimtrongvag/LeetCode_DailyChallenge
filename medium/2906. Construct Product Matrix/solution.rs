impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mod_val = 12345;

        // flatten grid
        let mut nums = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                nums.push(grid[i][j] as i64);
            }
        }

        let n = nums.len();

        // prefix[i] = product of nums[0..i-1]
        let mut prefix = vec![1i64; n];
        for i in 1..n {
            prefix[i] = (prefix[i - 1] * nums[i - 1]) % mod_val;
        }

        // suffix[i] = product of nums[i+1..n-1]
        let mut suffix = vec![1i64; n];
        for i in (0..n - 1).rev() {
            suffix[i] = (suffix[i + 1] * nums[i + 1]) % mod_val;
        }

        // result[i] = prefix[i] * suffix[i]
        let mut res_flat = vec![0i32; n];
        for i in 0..n {
            let val = (prefix[i] * suffix[i]) % mod_val;
            res_flat[i] = val as i32;
        }

        // reshape back to 2D
        let mut result = vec![vec![0; cols]; rows];
        let mut idx = 0;

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = res_flat[idx];
                idx += 1;
            }
        }

        result
    }
}