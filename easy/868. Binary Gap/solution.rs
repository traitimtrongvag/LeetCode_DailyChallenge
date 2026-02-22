impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut num = n;
        let mut last_pos: i32 = -1;  // position of previous '1'
        let mut max_gap = 0;
        let mut pos = 0;

        while num > 0 {
            if num & 1 == 1 {
                if last_pos != -1 {
                    max_gap = max_gap.max(pos - last_pos);
                }
                last_pos = pos;
            }

            num >>= 1;
            pos += 1;
        }

        max_gap
    }
}