impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let chars: Vec<char> = directions.chars().collect();
        let mut i = 0usize;
        let mut j = chars.len() as i32 - 1;

        while i < chars.len() && chars[i] == 'L' {
            i += 1;
        }
        while j >= 0 && chars[j as usize] == 'R' {
            j -= 1;
        }

        let mut count = 0;
        let mut k = i as i32;
        while k <= j {
            if chars[k as usize] != 'S' {
                count += 1;
            }
            k += 1;
        }

        count
    }
}