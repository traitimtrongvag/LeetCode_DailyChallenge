impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        // Sort the remaining elements (excluding the first),
        // then add the first element with the two smallest of the rest.
        let mut rest = nums[1..].to_vec();
        rest.sort();

        nums[0] + rest[0] + rest[1]
    }
}