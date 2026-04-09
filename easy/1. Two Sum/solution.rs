use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            let need = target - val;

            if let Some(&idx) = map.get(&need) {
                return vec![idx, i as i32];
            }

            map.insert(val, i as i32);
        }

        vec![]
    }
}
/*
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
   	 for i in 0..nums.len() {
        for j in i+1..nums.len(){
            if nums[i] + nums[j] == target{
                return vec![i as i32, j as i32]
            }
            
        }
  	 }
    vec![]
	}
}*/