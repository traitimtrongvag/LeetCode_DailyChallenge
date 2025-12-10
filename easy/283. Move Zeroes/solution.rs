impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut write_idx = 0;

        for read_idx in 0..nums.len() {
            if nums[read_idx] != 0 {
                nums.swap(read_idx, write_idx);
                write_idx += 1;
            }
        }
    }
}