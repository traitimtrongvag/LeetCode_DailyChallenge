impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut indeg = vec![0i32; n];
        let mut outdeg = vec![0i32; n];

        for t in trust {
            let a = (t[0] - 1) as usize;
            let b = (t[1] - 1) as usize;
            outdeg[a] += 1;
            indeg[b] += 1;
        }

        for i in 0..n {
            if indeg[i] == (n as i32 - 1) && outdeg[i] == 0 {
                return (i as i32) + 1;
            }
        }

        -1
    }
}