// Algorithm overview:
// Collect all possible segment lengths formed by fences and borders.
// Use max-heaps to always compare the largest horizontal and vertical lengths.
// Walk both heaps from large to small to find the biggest common length.
// If found, square it and return modulo 1e9+7. Otherwise, return -1.

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        // Build a max-heap of all possible distances between fences and borders.
        let into_set = |mut f: Vec<_>, m: _| -> std::collections::BinaryHeap<_> {
            // Sort fence positions to ensure increasing order.
            f.sort_unstable();

            // For each left fence (or border 1),
            // compute distances to every right fence (or boundary m).
            std::iter::once(&1)
                .chain(&f)
                .enumerate()
                .flat_map(|(i, l)| {
                    // Generate all valid segment lengths starting from l.
                    f[i..].iter().chain([&m]).map(move |r| r - l)
                })
                // Store distances in a max-heap.
                .collect()
        };

        // All possible vertical segment lengths.
        let mut h = into_set(h_fences, m);
        // All possible horizontal segment lengths.
        let mut v = into_set(v_fences, n);

        // Compare the largest remaining values from both heaps.
        while let (Some(hv), Some(vv)) = (h.peek(), v.peek()) {
            match hv.cmp(vv) {
                // Horizontal side is larger, discard it.
                std::cmp::Ordering::Less => {
                    v.pop();
                }
                // Vertical side is larger, discard it.
                std::cmp::Ordering::Greater => {
                    h.pop();
                }
                // Found the largest common side length.
                std::cmp::Ordering::Equal => {
                    // Return the square area modulo 1e9+7.
                    return (i64::from(*hv).pow(2) % 1_000_000_007) as _;
                }
            }
        }

        // No valid square can be formed.
        -1
    }
}