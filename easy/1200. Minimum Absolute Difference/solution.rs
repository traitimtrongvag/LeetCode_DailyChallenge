// Algorithm overview:
// Sort the array to bring close values next to each other.
// Scan once to find the minimum absolute difference between neighbors.
// Scan again to collect all adjacent pairs that match this minimum difference.
// Return all such pairs in ascending order.

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        // Sort the array to compare adjacent elements easily.
        let mut arr = arr;
        arr.sort();

        // Track the smallest difference found.
        let mut min_diff = i32::MAX;

        // First pass: find the minimum absolute difference.
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min_diff {
                min_diff = diff;
            }
        }

        // Store all pairs that have the minimum difference.
        let mut result = Vec::new();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min_diff {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }

        // Return all valid pairs.
        result
    }
}