impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut result = 0;
        for num1 in 1..n {
            for num2 in num1+1..n {
                let sqr = num1*num1 + num2*num2;
                let sqrt = (sqr as f32).sqrt();
                if sqrt.floor() == sqrt && sqrt as i32 <= n {
                    result += 1;
                }
            }
        }
        result * 2
    }
}