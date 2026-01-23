// Algorithm overview:
// Use a linked-list style structure to simulate merges efficiently.
// Track adjacent pair sums in a min-heap to always merge the smallest pair.
// Maintain an inversion counter to know when the sequence becomes sorted.
// On each merge, update neighbors, heap entries, and inversion count.
// Stop when the array becomes non-decreasing or no valid merges remain.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        // Handle trivial case.
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // Store values as i64 to avoid overflow during summation.
        let mut vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();

        // Next and previous pointers to simulate a linked list.
        let mut nexts: Vec<i32> = (0..n as i32).map(|i| i + 1).collect();
        let mut prevs: Vec<i32> = (0..n as i32).map(|i| i - 1).collect();

        // Marks nodes that have been merged and removed.
        let mut removed: Vec<bool> = vec![false; n];

        // Mark the end of the list.
        nexts[n - 1] = -1;

        // Min-heap storing (pair_sum, index_of_left_element).
        let mut heap: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::with_capacity(3 * n);

        // Count how many adjacent inversions exist.
        let mut unsorted_cnt: i32 = 0;

        // Initialize heap and inversion count.
        for i in 0..n - 1 {
            if vals[i] > vals[i + 1] {
                unsorted_cnt += 1;
            }
            heap.push(Reverse((vals[i] + vals[i + 1], i as i32)));
        }

        // Already sorted case.
        if unsorted_cnt == 0 {
            return 0;
        }

        // Number of merge operations performed.
        let mut moves = 0;

        // Continue until sequence becomes sorted.
        while unsorted_cnt > 0 {
            let Reverse((sum, u)) = match heap.pop() {
                Some(x) => x,
                None => break,
            };

            let u = u as usize;

            // Skip invalid or stale entries.
            if removed[u] {
                continue;
            }

            let v = nexts[u];
            if v == -1 {
                continue;
            }
            let v = v as usize;

            if removed[v] {
                continue;
            }

            // Skip outdated heap values.
            if vals[u] + vals[v] != sum {
                continue;
            }

            let p = prevs[u];
            let next_v = nexts[v];

            moves += 1;

            // Remove old inversion contributions.
            if p != -1 && vals[p as usize] > vals[u] {
                unsorted_cnt -= 1;
            }
            if vals[u] > vals[v] {
                unsorted_cnt -= 1;
            }
            if next_v != -1 && vals[v] > vals[next_v as usize] {
                unsorted_cnt -= 1;
            }

            // Merge v into u.
            vals[u] = sum;
            nexts[u] = next_v;
            if next_v != -1 {
                prevs[next_v as usize] = u as i32;
            }
            removed[v] = true;

            // Add new inversion contributions.
            if p != -1 && vals[p as usize] > vals[u] {
                unsorted_cnt += 1;
            }
            if next_v != -1 && vals[u] > vals[next_v as usize] {
                unsorted_cnt += 1;
            }

            // Stop early if sorted.
            if unsorted_cnt == 0 {
                break;
            }

            // Push updated neighbor pairs into heap.
            if p != -1 {
                heap.push(Reverse((vals[p as usize] + vals[u], p)));
            }
            if next_v != -1 {
                heap.push(Reverse((vals[u] + vals[next_v as usize], u as i32)));
            }
        }

        moves
    }
}