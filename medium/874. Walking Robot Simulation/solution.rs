// simulate robot moves with obstacles
// track direction using dx/dy arrays

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut max_dist = 0;

        let dirs = [(0,1), (1,0), (0,-1), (-1,0)]; // N,E,S,W
        let mut d = 0;

        let obs: HashSet<(i32,i32)> = obstacles.into_iter()
            .map(|v| (v[0], v[1]))
            .collect();

        for &c in &commands {
            match c {
                -1 => d = (d + 1) % 4, // turn right
                -2 => d = (d + 3) % 4, // turn left
                n => {
                    for _ in 0..n {
                        let nx = x + dirs[d].0;
                        let ny = y + dirs[d].1;
                        if obs.contains(&(nx, ny)) { break; }
                        x = nx;
                        y = ny;
                        max_dist = max_dist.max(x*x + y*y);
                    }
                }
            }
        }

        max_dist
    }
}