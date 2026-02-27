use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::Included;

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let n = s.len() as i32;
        let zero_count = s.chars().filter(|&c| c == '0').count() as i32;

        // Track unvisited states by parity of zero count
        let mut odd_states: BTreeSet<i32> = (1..=n).step_by(2).collect();
        let mut even_states: BTreeSet<i32> = (0..=n).step_by(2).collect();

        odd_states.remove(&zero_count);
        even_states.remove(&zero_count);

        // Parity check: impossible to escape odd state
        if zero_count % 2 == 1 && k % 2 == 0 {
            return -1;
        }

        let mut queue = VecDeque::new();
        queue.push_back(zero_count);

        let mut steps = 0;
        let mut max_reachable = std::cmp::min(2 * k, zero_count + k);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                if current == 0 {
                    return steps;
                }

                max_reachable = max_reachable.min(current + k);

                // Select how many zeros we flip
                let min_pick = std::cmp::max(0, k - (n - current));
                let max_pick = std::cmp::min(k, current);

                let low = current + k - 2 * max_pick;
                let high = std::cmp::max(
                    low + 1,
                    std::cmp::min(current + k - 2 * min_pick, max_reachable),
                );

                // Reachable next states depend on parity
                let candidates = if (current + k) % 2 == 0 {
                    even_states.range((Included(&low), Included(&high)))
                } else {
                    odd_states.range((Included(&low), Included(&high)))
                };

                let next_states: Vec<i32> = candidates.copied().collect();

                for next in next_states {
                    queue.push_back(next);
                    if next % 2 == 0 {
                        even_states.remove(&next);
                    } else {
                        odd_states.remove(&next);
                    }
                }
            }
            steps += 1;
        }

        -1
    }
}