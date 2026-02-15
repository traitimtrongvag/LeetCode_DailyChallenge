impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
       nums.sort();
    let mut sum = 0;

    for j in (0..nums.len()).step_by(2) {
        sum += nums[j]
    }
    sum  
    }
}