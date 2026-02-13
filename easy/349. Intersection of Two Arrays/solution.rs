impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for &value in nums1.iter() {
            if Self::contains(&nums2, value) {
                if !Self::contains(&result, value) {
                    result.push(value);
                }
            }
        }

        result
    }

    fn contains(arr: &Vec<i32>, target: i32) -> bool {
        for &value in arr.iter() {
            if value == target {
                return true;
            }
        }
        false
    }
}