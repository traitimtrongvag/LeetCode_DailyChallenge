impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        
        fn first_digit(mut num: i32) -> i32 {
            while num >= 10 {
                num /= 10;
            }
            num
        }
        
        let mut count = 0;
        let n = nums.len();
        
        for i in 0..n {
            for j in (i + 1)..n {
                let first = first_digit(nums[i]);
                let last = nums[j] % 10;
                
                if gcd(first, last) == 1 {
                    count += 1;
                }
            }
        }
        
        count
    }
}