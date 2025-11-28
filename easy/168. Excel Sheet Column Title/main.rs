// 168. Excel Sheet Column Title
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        let mut n = column_number;
        
        while n > 0 {
            n -= 1;
            result.insert(0, (b'A' + (n % 26) as u8) as char);
            n /= 26;
        }
        result
    }
}