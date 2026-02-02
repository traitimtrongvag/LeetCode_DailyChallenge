use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;
        let mut min_sum = i64::MAX;

        // max_heap keeps the smallest (k-1) chosen values in the current window
        // min_heap keeps the remaining candidates
        let mut max_heap = BTreeSet::<(i32, usize)>::new();
        let mut min_heap = BTreeSet::<(i32, usize)>::new();

        let mut curr_sum = 0i64;

        for i in 1..n {
            // Insert new element into appropriate set
            if max_heap.len() < k - 1 || nums[i] < max_heap.last().unwrap().0 {
                max_heap.insert((nums[i], i));
                curr_sum += nums[i] as i64;

                // If we exceed k-1 elements, move the largest to min_heap
                if max_heap.len() == k {
                    let num = max_heap.pop_last().unwrap();
                    curr_sum -= num.0 as i64;
                    min_heap.insert(num);
                }
            } else {
                min_heap.insert((nums[i], i));
            }

            // If we have enough elements, update answer
            if max_heap.len() >= k - 1 {
                min_sum = std::cmp::min(min_sum, curr_sum);
            }

            // Slide window: remove elements that are out of range
            if i - 1 >= dist {
                let num = (nums[i - dist], i - dist);

                if max_heap.contains(&num) {
                    curr_sum -= num.0 as i64;
                    max_heap.remove(&num);

                    // Rebalance by taking the smallest from min_heap
                    if !min_heap.is_empty() {
                        let p_num = min_heap.pop_first().unwrap();
                        curr_sum += p_num.0 as i64;
                        max_heap.insert(p_num);
                    }
                } else {
                    min_heap.remove(&num);
                }
            }
        }

        // Final result includes nums[0]
        nums[0] as i64 + min_sum
    }
}