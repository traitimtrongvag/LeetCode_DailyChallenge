impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut list: Vec<String> = Vec::new();

        fn backtrack(cur: &mut String, n: usize, list: &mut Vec<String>) {
            if cur.len() == n {
                list.push(cur.clone());
                return;
            }

            for c in ['a', 'b', 'c'] {
                if cur.is_empty() || cur.chars().last().unwrap() != c {
                    cur.push(c);
                    backtrack(cur, n, list);
                    cur.pop();
                }
            }
        }

        let mut cur = String::new();
        backtrack(&mut cur, n, &mut list);

        let k = (k - 1) as usize;
        if k >= list.len() {
            return "".to_string();
        }

        list[k].clone()
    }
}