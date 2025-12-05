use std::collections::HashMap;

impl Solution {
    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }
        let mut i = 3;
        while i * i <= num {
            if num % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
    
    pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut freq = HashMap::new();
        
        let dirs = [
            (0, 1), (1, 0), (0, -1), (-1, 0),
            (1, 1), (1, -1), (-1, 1), (-1, -1)
        ];
        
        for i in 0..m {
            for j in 0..n {
                for &(dx, dy) in &dirs {
                    let mut x = i as i32;
                    let mut y = j as i32;
                    let mut num = 0;
                    while x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                        num = num * 10 + mat[x as usize][y as usize];
                        if num > 10 && Self::is_prime(num) {
                            *freq.entry(num).or_insert(0) += 1;
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
        
        let mut max_freq = 0;
        let mut result = -1;
        for (&num, &count) in &freq {
            if count > max_freq || (count == max_freq && num > result) {
                max_freq = count;
                result = num;
            }
        }
        result
    }
}