impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        // sort to enable two-pointer window
        nums.sort();
        let n = nums.len();

        let mut p2: usize = 0;
        // initialize best removal count with worst case
        let mut best: usize = n;

        for p1 in 0..n {
            // current upper bound for valid elements
            let limit = nums[p1] as i64 * k as i64;

            // expand right pointer while elements are valid
            while p2 < n && nums[p2] as i64 <= limit {
                p2 += 1;
            }

            // elements removed = left outside window + right outside window
            let removed = p1 + (n - p2);
            if removed < best {
                best = removed;
            }

            // no need to continue if right pointer reached end
            if p2 == n {
                break;
            }
        }
        best as i32
    }
}