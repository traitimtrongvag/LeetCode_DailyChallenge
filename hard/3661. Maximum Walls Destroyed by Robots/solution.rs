use std::collections::HashMap;

impl Solution {
    pub fn max_walls(r: Vec<i32>, d: Vec<i32>, w: Vec<i32>) -> i32 {
        let n = r.len();

        // map robot -> distance
        let mut mp = HashMap::new();
        for i in 0..n {
            mp.insert(r[i], d[i]);
        }

        let mut a = r.clone();
        let mut b = w.clone();
        a.sort_unstable();
        b.sort_unstable();

        let mut l = vec![0; n];
        let mut rr = vec![0; n];
        let mut mid = vec![0; n];

        for i in 0..n {
            let pos_r = a[i];
            let dist = mp[&pos_r];

            let p1 = b.partition_point(|&x| x <= pos_r);

            let l_bound = if i > 0 {
                (pos_r - dist).max(a[i - 1] + 1)
            } else {
                pos_r - dist
            };
            let p2 = b.partition_point(|&x| x < l_bound);
            l[i] = p1 - p2;

            let r_bound = if i + 1 < n {
                (pos_r + dist).min(a[i + 1] - 1)
            } else {
                pos_r + dist
            };
            let p3 = b.partition_point(|&x| x <= r_bound);
            let p4 = b.partition_point(|&x| x < pos_r);
            rr[i] = p3 - p4;

            if i > 0 {
                let p5 = b.partition_point(|&x| x < a[i - 1]);
                mid[i] = p1 - p5;
            }
        }

        // dp
        let mut L = l[0];
        let mut R = rr[0];

        for i in 1..n {
            let nL = (L + l[i]).max(
                R - rr[i - 1] + (l[i] + rr[i - 1]).min(mid[i])
            );
            let nR = (L + rr[i]).max(R + rr[i]);

            L = nL;
            R = nR;
        }

        (L.max(R)) as i32
    }
}