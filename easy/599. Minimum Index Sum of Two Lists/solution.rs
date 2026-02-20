use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(
        list1: Vec<String>,
        list2: Vec<String>,
    ) -> Vec<String> {

        // Map: restaurant name -> index in list1
        let mut map = HashMap::new();
        for (i, name) in list1.iter().enumerate() {
            map.insert(name, i);
        }

        let mut result = Vec::new();
        let mut min_sum = usize::MAX;

        // Traverse list2 and check if restaurant exists in list1
        for (j, name) in list2.iter().enumerate() {
            if let Some(&i) = map.get(name) {
                let index_sum = i + j;

                if index_sum < min_sum {
                    min_sum = index_sum;
                    result.clear();
                    result.push(name.clone());
                } else if index_sum == min_sum {
                    result.push(name.clone());
                }
            }
        }

        result
    }
}