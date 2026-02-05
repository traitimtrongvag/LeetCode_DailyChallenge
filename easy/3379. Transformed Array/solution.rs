impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, &x)| {
                if x == 0 {
                    0
                } else {
                    let idx = (i as i32 + x % n as i32 + n as i32) % n as i32;
                    nums[idx as usize]
                }
            })
            .collect()
    }
}