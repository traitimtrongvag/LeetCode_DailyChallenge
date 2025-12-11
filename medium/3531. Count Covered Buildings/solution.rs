const X: usize = 0;
const Y: usize = 1;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut y_minmax_by_x = vec![MinMax::default(); n as usize + 1];
        let mut x_minmax_by_y = vec![MinMax::default(); n as usize + 1];

        for b in &buildings {
            x_minmax_by_y[b[Y] as usize].update(b[X]);
            y_minmax_by_x[b[X] as usize].update(b[Y]);
        }

        let mut n_covered = 0;

        for b in buildings {
            if x_minmax_by_y[b[Y] as usize].covers(b[X]) 
                && y_minmax_by_x[b[X] as usize].covers(b[Y]) {
                n_covered += 1;
            }
        }

        n_covered
    }
}

#[derive(Clone, Copy)]
struct MinMax {
    min: i32,
    max: i32,
}
impl MinMax {
    fn new() -> Self { 
        Self { min: i32::MAX, max: i32::MIN } 
    }
    fn update(&mut self, v: i32) {
        self.min = self.min.min(v);
        self.max = self.max.max(v);
    }
    fn covers(&self, v: i32) -> bool {
        self.min < v && v < self.max
    }
}
impl Default for MinMax {
    fn default() -> Self {
        Self::new()
    }
}