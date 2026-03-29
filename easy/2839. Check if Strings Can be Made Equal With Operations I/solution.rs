impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();

        // even indices: 0,2  | odd indices: 1,3
        // we can freely swap inside each group

        let mut even1 = vec![a[0], a[2]];
        let mut even2 = vec![b[0], b[2]];

        let mut odd1 = vec![a[1], a[3]];
        let mut odd2 = vec![b[1], b[3]];

        even1.sort();
        even2.sort();
        odd1.sort();
        odd2.sort();

        even1 == even2 && odd1 == odd2
    }
}