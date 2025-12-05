use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut maxv = 0;
        for &x in &nums { if x > maxv { maxv = x; } }
        let maxv = maxv as usize;

        let mut spf = vec![0usize; maxv + 1];
        for i in 2..=maxv {
            if spf[i] == 0 {
                spf[i] = i;
                if i * i <= maxv {
                    let mut j = i * i;
                    while j <= maxv {
                        if spf[j] == 0 { spf[j] = i; }
                        j += i;
                    }
                }
            }
        }

        fn factors(mut x: usize, spf: &Vec<usize>) -> Vec<usize> {
            let mut r = Vec::new();
            while x > 1 {
                let p = spf[x];
                r.push(p);
                while x % p == 0 { x /= p; }
            }
            r
        }

        let mut mp: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut facs = Vec::with_capacity(n);
        for i in 0..n {
            let fs = factors(nums[i] as usize, &spf);
            for &p in &fs {
                mp.entry(p).or_default().push(i);
            }
            facs.push(fs);
        }

        let mut vis = vec![false; n];
        let mut q = VecDeque::new();
        vis[0] = true;
        q.push_back((0usize, 0i32));

        while let Some((i, d)) = q.pop_front() {
            if i == n - 1 { return d; }

            if i + 1 < n && !vis[i + 1] {
                vis[i + 1] = true;
                q.push_back((i + 1, d + 1));
            }
            if i > 0 && !vis[i - 1] {
                vis[i - 1] = true;
                q.push_back((i - 1, d + 1));
            }

            let v = nums[i] as usize;

            if spf[v] == v {
                for &p in &facs[i] {
                    if let Some(list) = mp.get_mut(&p) {
                        for &j in list.iter() {
                            if !vis[j] {
                                vis[j] = true;
                                q.push_back((j, d + 1));
                            }
                        }
                        list.clear();
                    }
                }
            }
        }

        -1
    }
}