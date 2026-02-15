impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m*n
        }

        let (mut a_min, mut b_min) = (m, n);
        for op in ops {
            a_min = a_min.min(op[0]);
            b_min = b_min.min(op[1]);
        }

        a_min * b_min
    }
}