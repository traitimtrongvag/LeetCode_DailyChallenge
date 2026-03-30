impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let n = s1.len();

        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();

        let mut even1 = Vec::new();
        let mut even2 = Vec::new();

        let mut odd1 = Vec::new();
        let mut odd2 = Vec::new();

        // split by parity index
        for i in 0..n {
            if i % 2 == 0 {
                even1.push(a[i]);
                even2.push(b[i]);
            } else {
                odd1.push(a[i]);
                odd2.push(b[i]);
            }
        }

        // sort each group
        even1.sort();
        even2.sort();
        odd1.sort();
        odd2.sort();

        even1 == even2 && odd1 == odd2
    }
}