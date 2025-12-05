use std::collections::BTreeMap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = Vec::new();
        for b in &buildings {
            points.push((b[0], -b[2]));
            points.push((b[1], b[2]));
        }
        points.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        
        let mut heights = BTreeMap::new();
        heights.insert(0, 1);
        let mut result = Vec::new();
        let mut prev = 0;
        
        for (x, h) in points {
            if h < 0 {
                *heights.entry(-h).or_insert(0) += 1;
            } else {
                let entry = heights.entry(h).or_insert(0);
                *entry -= 1;
                if *entry == 0 {
                    heights.remove(&h);
                }
            }
            
            let curr = *heights.keys().rev().next().unwrap();
            if curr != prev {
                result.push(vec![x, curr]);
                prev = curr;
            }
        }
        result
    }
}