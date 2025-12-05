impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as i64;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        
        let mut components = 0;
        Self::dfs(0, n, &graph, &values, k, &mut components);
        components
    }
    
    fn dfs(node: usize, parent: usize, graph: &Vec<Vec<usize>>, values: &Vec<i32>, k: i64, components: &mut i32) -> i64 {
        let mut sum = values[node] as i64;
        for &neighbor in &graph[node] {
            if neighbor != parent {
                sum += Self::dfs(neighbor, node, graph, values, k, components);
            }
        }
        if sum % k == 0 {
            *components += 1;
            return 0;
        }
        sum
    }
}