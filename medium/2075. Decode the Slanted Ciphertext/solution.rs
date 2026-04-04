// build matrix row-wise, then read diagonals

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        if rows == 0 { return String::new(); }

        let n = encoded_text.len();
        let cols = n / rows;

        let s: Vec<char> = encoded_text.chars().collect();
        let mut res = String::new();

        // start from each column in first row
        for start in 0..cols {
            let mut r = 0;
            let mut c = start;

            // go diagonally down-right
            while r < rows && c < cols {
                res.push(s[r * cols + c]);
                r += 1;
                c += 1;
            }
        }

        // trim trailing spaces
        while res.ends_with(' ') {
            res.pop();
        }

        res
    }
}