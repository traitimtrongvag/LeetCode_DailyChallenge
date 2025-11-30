// 1590. Make Sum Divisible by P
use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let total: i64 = nums.iter().map(|&x| x as i64).sum();
        let target = (total % p as i64) as i32;
        if target == 0 {
            return 0;
        }
        
        let mut prefix = HashMap::new();
        prefix.insert(0, -1);
        let mut sum = 0;
        let mut min_len = nums.len() as i32;
        
        for (i, &num) in nums.iter().enumerate() {
            sum = (sum + num) % p;
            let need = (sum - target + p) % p;
            
            if let Some(&idx) = prefix.get(&need) {
                min_len = min_len.min(i as i32 - idx);
            }
            prefix.insert(sum, i as i32);
        }
        
        if min_len == nums.len() as i32 { -1 } else { min_len }
    }
}