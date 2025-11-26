// 31. Next Permutation
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut i = n as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }
        if i >= 0 {
            let mut j = n - 1;
            while nums[j] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(i as usize, j);
        }
        let mut left = i as usize + 1;
        let mut right = n - 1;
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}