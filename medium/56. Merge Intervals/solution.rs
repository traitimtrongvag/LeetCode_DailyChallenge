impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        
        let mut intervals = intervals;
        intervals.sort_by_key(|v| v[0]);
        
        let mut merged = Vec::new();
        merged.push(intervals[0].clone());
        
        for i in 1..intervals.len() {
            let last = merged.last_mut().unwrap();
            if intervals[i][0] <= last[1] {
                last[1] = last[1].max(intervals[i][1]);
            } else {
                merged.push(intervals[i].clone());
            }
        }
        
        merged
    }
}