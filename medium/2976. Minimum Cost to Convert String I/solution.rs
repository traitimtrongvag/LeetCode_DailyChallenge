impl Solution {
    // Build the minimum conversion cost between every pair of characters.
    // Initialize all costs as INF, and zero for same-character conversion.
    // Apply Floyd–Warshall to compute the cheapest indirect conversions.
    fn build_min_price(
        original: &Vec<char>,
        changed: &Vec<char>,
        cost: &Vec<i32>,
    ) -> [[i32; 28]; 28] {
        const INF: i32 = i32::MAX / 2;

        // price[i][j] stores the minimum cost to convert char i to char j.
        let mut price = [[INF; 28]; 28];

        // Cost to convert a character to itself is zero.
        for i in 0..28 {
            price[i][i] = 0;
        }

        // Fill direct conversion costs.
        for i in 0..original.len() {
            let original_idx = (original[i] as u8 - b'a') as usize;
            let changed_idx = (changed[i] as u8 - b'a') as usize;

            // Keep the smallest cost if multiple rules exist.
            price[original_idx][changed_idx] =
                price[original_idx][changed_idx].min(cost[i]);
        }

        // Floyd–Warshall to relax all intermediate paths.
        for k in 0..28 {
            for i in 0..28 {
                for j in 0..28 {
                    price[i][j] = price[i][j].min(price[i][k] + price[k][j]);
                }
            }
        }

        price
    }

    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const INF: i32 = i32::MAX / 2;

        // Precompute minimum conversion costs.
        let price = Solution::build_min_price(&original, &changed, &cost);

        // Accumulate total cost.
        let mut ans = 0i64;

        let src = source.as_bytes();
        let tgt = target.as_bytes();

        // Compare characters position by position.
        for i in 0..src.len() {
            if src[i] != tgt[i] {
                let p = price[(src[i] - b'a') as usize][(tgt[i] - b'a') as usize];

                // If conversion is impossible, return -1.
                if p == INF {
                    return -1;
                }

                ans += p as i64;
            }
        }

        ans
    }
}