use std::collections::HashMap;

impl Solution {
    pub fn two_sum(g_nums: Vec<i32>, g_target: i32) -> Vec<i32> {
        let mut g_map: HashMap<i32, i32> = HashMap::new();

        for (n_i, n_val) in g_nums.iter().enumerate() {
            let n_need = g_target - n_val;

            if let Some(&n_idx) = g_map.get(&n_need) {
                return vec![n_idx, n_i as i32];
            }

            g_map.insert(*n_val, n_i as i32);
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