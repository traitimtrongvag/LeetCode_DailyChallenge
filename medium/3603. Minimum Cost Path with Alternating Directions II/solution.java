class Solution {
    public long minCost(int m, int n, int[][] waitCost) {
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.offer(new long[]{1, 0, 0}); // {cost, x, y}

        long[][] costMatrix = new long[m][n];
        for (long[] row : costMatrix)
            Arrays.fill(row, Long.MAX_VALUE);
        
        costMatrix[0][0] = 1;

        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long cost = top[0];
            int x = (int)top[1];
            int y = (int)top[2];

            if (cost > costMatrix[x][y]) continue;

            if (x == m - 1 && y == n - 1)
                return cost - waitCost[0][0];

            // Move Down
            int nx = x + 1, ny = y;
            if (nx < m) {
                long newCost = cost + waitCost[x][y] + (long)(nx + 1) * (ny + 1);
                if (costMatrix[nx][ny] > newCost) {
                    costMatrix[nx][ny] = newCost;
                    pq.offer(new long[]{newCost, nx, ny});
                }
            }

            // Move Right
            nx = x; ny = y + 1;
            if (ny < n) {
                long newCost = cost + waitCost[x][y] + (long)(nx + 1) * (ny + 1);
                if (costMatrix[nx][ny] > newCost) {
                    costMatrix[nx][ny] = newCost;
                    pq.offer(new long[]{newCost, nx, ny});
                }
            }
        }

        return -1;
    }
}