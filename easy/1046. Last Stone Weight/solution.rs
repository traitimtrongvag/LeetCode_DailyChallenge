use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        // Max-heap to always take the two heaviest stones
        let mut heap = BinaryHeap::new();

        for stone in stones {
            heap.push(stone);
        }

        // Keep smashing while there are at least two stones
        while heap.len() > 1 {
            let first = heap.pop().unwrap();
            let second = heap.pop().unwrap();

            // If weights are different, push the remaining weight back
            if first != second {
                heap.push(first - second);
            }
        }

        // If one stone remains, return its weight, otherwise return 0
        heap.pop().unwrap_or(0)
    }
}