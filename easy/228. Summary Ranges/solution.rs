impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let n = nums.len();
        if n == 0 {
            return result;
        }
        
        let mut start = nums[0];
        for i in 1..=n {
            if i < n && nums[i] == nums[i - 1] + 1 {
                continue;
            } else {
                if start == nums[i - 1] {
                    result.push(start.to_string());
                } else {
                    result.push(format!("{}->{}", start, nums[i - 1]));
                }
                if i < n {
                    start = nums[i];
                }
            }
        }
        result
    }
}