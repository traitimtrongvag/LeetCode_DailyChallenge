impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as u64;

            let mut i = l;

            while i <= r {
                let x = nums[i] as u64 * v;
                let a = x % (1_000_000_000 + 7);
                nums[i] = a as i32;
                i += k;
            }
        }

        let result = nums.iter().fold(0, |acc, &n| acc ^ n);
        result
    }
}