impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        // size mismatch → return original
        if m * n != (r * c) as usize {
            return mat;
        }

        let mut res = vec![vec![0; c as usize]; r as usize];

        // treat matrix as 1D index
        for idx in 0..(m * n) {
            let val = mat[idx / n][idx % n];
            res[idx / c as usize][idx % c as usize] = val;
        }

        res
    }
}