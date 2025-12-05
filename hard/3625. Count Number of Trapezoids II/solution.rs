use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut t: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut v: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let n = points.len();

        fn add(map: &mut HashMap<i32, HashMap<i32, i32>>, key: i32, des: i32) {
            let inner = map.entry(key).or_insert_with(HashMap::new);
            *inner.entry(des).or_insert(0) += 1;
        }

        for i in 0..n {
            let x1 = points[i][0];
            let y1 = points[i][1];
            for j in i+1..n {
                let x2 = points[j][0];
                let y2 = points[j][1];

                let mut dx = x2 - x1;
                let mut dy = y2 - y1;

                if dx < 0 || (dx == 0 && dy < 0) {
                    dx = -dx;
                    dy = -dy;
                }

                let g = gcd(dx, dy.abs());
                let sx = dx / g;
                let sy = dy / g;

                let des = sx * y1 - sy * x1;

                let key1 = (sx << 12) | (sy + 2000);
                let key2 = (dx << 12) | (dy + 2000);

                add(&mut t, key1, des);
                add(&mut v, key2, des);
            }
        }

        count(&t) - count(&v)/2
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

fn count(mp: &HashMap<i32, HashMap<i32, i32>>) -> i32 {
    let mut ans: i64 = 0;

    for inner in mp.values() {
        let total: i64 = inner.values().map(|&v| v as i64).sum();
        let mut rem = total;
        for &val in inner.values() {
            rem -= val as i64;
            ans += val as i64 * rem;
        }
    }

    ans as i32
}