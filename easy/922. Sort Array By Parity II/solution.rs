impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 1;
        
        while even < nums.len() && odd < nums.len() {
            if nums[even] % 2 == 0 {
                even += 2;
            } else {
                nums.swap(even, odd);
                odd += 2;
            }
        }
        
        nums
    }
}