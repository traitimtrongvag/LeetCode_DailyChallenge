impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max_val = *nums.iter().max().unwrap() as usize;

        let mut max_len: usize = 0;
        let mut count = vec![0i32; max_val + 1];
        let mut last_seen = vec![n; max_val + 1];
        let mut next_idx = vec![n; n];
        let mut distinct = [0i32; 2];

        for (i, &x) in nums.iter().enumerate() {
            Self::update(x, 1, &mut count, &mut distinct);
            if last_seen[x as usize] < n {
                next_idx[last_seen[x as usize]] = i;
            }
            last_seen[x as usize] = i;
        }

        let mut end = n;

        for start in 0..n - 1 {
            if n - start <= max_len {
                break;
            }

            while end - start > max_len {
                if distinct[0] == distinct[1] {
                    max_len = end - start;
                    break;
                }
                end -= 1;
                Self::update(nums[end], -1, &mut count, &mut distinct);
            }

            while end < next_idx[start] {
                Self::update(nums[end], 1, &mut count, &mut distinct);
                end += 1;
            }

            Self::update(nums[start], -1, &mut count, &mut distinct);
        }

        max_len as i32
    }

    // val = +1 (add element) or -1 (remove element)
    fn update(x: i32, val: i32, count: &mut Vec<i32>, distinct: &mut [i32; 2]) {
        let idx = x as usize;
        count[idx] += val;

        if count[idx] == 0 && val < 0 {
            distinct[idx & 1] -= 1;
        } else if count[idx] == val && val > 0 {
            distinct[idx & 1] += 1;
        }
    }
}