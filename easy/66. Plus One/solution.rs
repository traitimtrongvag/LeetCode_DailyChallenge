impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
	    let mut i = digits.len();
	    while i > 0 {
	        i -= 1;
	        if digits[i] < 9 {
	            digits[i] += 1;
	            return digits;
	        }
	        digits[i] = 0;
	    }
	    let mut result = vec![1];
	    result.extend(digits);
	    result
	}
}