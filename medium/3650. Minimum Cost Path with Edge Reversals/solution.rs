use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
      const INF:i32 = i32::MAX;
      let n = n as usize;

      let mut adj:Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
      for e in  edges.iter() {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        adj[u].push((v, w));
        adj[v].push((u, 2*w));
      }

      let mut dist = vec![INF; n];
      
      let (Q, T) = (0 as usize, n - 1);
      dist[Q] = 0;

      let mut pq = BinaryHeap::new();
      // use cmp::Reverse to get a MinHeap
      pq.push(Reverse((dist[Q], Q)));
      while let Some(Reverse((d, u))) = pq.pop() {
        if u == T {
          return dist[u];
        }
        if d > dist[u] { continue; } // filter out any sub-optimal paths to u

        for &(v, w) in adj[u].iter() {
          if dist[u] + w < dist[v] { // update the distances if we find a better path to v
            dist[v] = dist[u] + w;
            pq.push(Reverse((dist[v], v)));
          }
        }
      }

      -1 // no path from Q to T
    }
}