// idea: https://youtu.be/xWm_Mr0oDHE?si=gGcWxhS87e6IFo9t
class Solution {
    public int numIslands(char[][] grid) {
        int rows = grid.length;
        int cols = grid[0].length;

        boolean[][] visited = new boolean[rows][cols];
        int count = 0;

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] == '1' && !visited[r][c]) {
                    count++;
                    dfs(grid, visited, r, c);
                }
            }
        }
        return count;
    }

    private void dfs(char[][] grid, boolean[][] visited, int r, int c) {
        int rows = grid.length;
        int cols = grid[0].length;

        if (r < 0 || c < 0 || r >= rows || c >= cols) return;
        if (grid[r][c] == '0' || visited[r][c]) return;

        visited[r][c] = true;

        dfs(grid, visited, r + 1, c);
        dfs(grid, visited, r - 1, c);
        dfs(grid, visited, r, c + 1);
        dfs(grid, visited, r, c - 1);
    }
}