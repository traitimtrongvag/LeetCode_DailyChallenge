// 35. Search Insert Position
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
	    let mut left = 0;
	    let mut right = nums.len() as i32 - 1;
	
	    while left <= right {
	        let mid = left + (right - left) / 2;
	        let value = nums[mid as usize];
	
	        if value == target {
	            return mid;
	        } else if value < target {
	            left = mid + 1;
	        } else {
	            right = mid - 1;
	        }
	    }
	
	    left
	}
}