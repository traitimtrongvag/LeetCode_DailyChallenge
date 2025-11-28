// 2872. Maximum Number of K-Divisible Components
long long dfs(int node, int parent, int** graph, int* graphSize, int* values, int k, int* components) {
    long long sum = values[node];
    for (int i = 0; i < graphSize[node]; i++) {
        int neighbor = graph[node][i];
        if (neighbor != parent) {
            sum += dfs(neighbor, node, graph, graphSize, values, k, components);
        }
    }
    if (sum % k == 0) {
        (*components)++;
        return 0;
    }
    return sum;
}

int maxKDivisibleComponents(int n, int** edges, int edgesSize, int* edgesColSize, int* values, int valuesSize, int k) {
    int** graph = (int**)malloc(n * sizeof(int*));
    int* graphSize = (int*)calloc(n, sizeof(int));
    
    for (int i = 0; i < edgesSize; i++) {
        int u = edges[i][0], v = edges[i][1];
        graphSize[u]++;
        graphSize[v]++;
    }
    
    for (int i = 0; i < n; i++) {
        graph[i] = (int*)malloc(graphSize[i] * sizeof(int));
        graphSize[i] = 0;
    }
    
    for (int i = 0; i < edgesSize; i++) {
        int u = edges[i][0], v = edges[i][1];
        graph[u][graphSize[u]++] = v;
        graph[v][graphSize[v]++] = u;
    }
    
    int components = 0;
    dfs(0, -1, graph, graphSize, values, k, &components);
    
    for (int i = 0; i < n; i++) {
        free(graph[i]);
    }
    free(graph);
    free(graphSize);
    
    return components;
}