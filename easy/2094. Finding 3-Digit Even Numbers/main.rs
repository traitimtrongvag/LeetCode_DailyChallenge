// 2094. Finding 3-Digit Even Numbers
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 10];
        for &d in &digits {
            count[d as usize] += 1;
        }
        
        let mut result = Vec::new();
        for num in (100..=998).step_by(2) {
            let mut temp_count = vec![0; 10];
            let mut n = num;
            while n > 0 {
                temp_count[(n % 10) as usize] += 1;
                n /= 10;
            }
            
            let mut valid = true;
            for i in 0..10 {
                if temp_count[i] > count[i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                result.push(num);
            }
        }
        result
    }
}