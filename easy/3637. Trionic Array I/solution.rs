impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut i = 0;

        // strictly increasing
        while i + 1 < n && nums[i] < nums[i + 1] {
            i += 1;
        }
        if i == 0 || i == n - 1 {
            return false;
        }

        // strictly decreasing
        let p = i;
        while i + 1 < n && nums[i] > nums[i + 1] {
            i += 1;
        }
        if i == p || i == n - 1 {
            return false;
        }

        // strictly increasing to the end
        while i + 1 < n && nums[i] < nums[i + 1] {
            i += 1;
        }

        i == n - 1
    }
}