// 3079. Find the Sum of Encrypted Integers
impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        
        for &num in nums.iter() {
            total += Self::encrypt(num);
        }
        
        total
    }
    
    fn encrypt(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        
        let mut max_digit = 0;
        let mut count = 0;
        let mut n = num;
        
        // Find the maximum digit and count digits
        while n > 0 {
            let digit = n % 10;
            if digit > max_digit {
                max_digit = digit;
            }
            count += 1;
            n /= 10;
        }
        
        // Build the encrypted number
        let mut result = 0;
        for _ in 0..count {
            result = result * 10 + max_digit;
        }
        
        result
    }
}