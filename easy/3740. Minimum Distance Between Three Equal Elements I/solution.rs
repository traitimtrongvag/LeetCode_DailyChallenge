impl Solution {
    fn calc(nums: &Vec<i32>, target: i32) -> i32 {
        let mut pos = Vec::new();

        for i in 0..nums.len() {
            if nums[i] == target {
                pos.push(i as i32);
            }
        }

        let mut res = i32::MAX;

        for i in 0..=pos.len() - 3 {
            let a = pos[i];
            let b = pos[i + 1];
            let c = pos[i + 2];

            let cur = (a - b).abs() + (b - c).abs() + (c - a).abs();
            res = res.min(cur);
        }

        res
    }

    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut freq = HashMap::new();
        let mut res = i32::MAX;

        for &x in &nums {
            let cnt = freq.entry(x).or_insert(0);
            *cnt += 1;

            if *cnt == 3 {
                let val = Self::calc(&nums, x);
                res = res.min(val);
            }
        }

        if res == i32::MAX { -1 } else { res }
    }
}