impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut idx_arr = [-1; 1001];
        for (n, i) in arr2.into_iter().zip(0..) {
            idx_arr[n as usize] = i;
        }
        arr1.sort_unstable_by(|a, b| {
            use std::cmp::Ordering::*;
            match (idx_arr[*a as usize], idx_arr[*b as usize]) {
                (-1, -1) => a.cmp(b),
                (-1,  _) => Greater,
                ( _, -1) => Less,
                ( i,  j) => i.cmp(&j),
            }
        });
        arr1
    }
}