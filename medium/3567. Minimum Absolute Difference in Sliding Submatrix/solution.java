class Solution {
    public int[][] minAbsDiff(int[][] grid, int k) {
        int rows = grid.length;
        int cols = grid[0].length;

        int[][] result = new int[rows - k + 1][cols - k + 1];

        for (int r = 0; r <= rows - k; r++) {
            for (int c = 0; c <= cols - k; c++) {

                int[] values = new int[k * k];
                int idx = 0;

                // collect k x k block
                for (int i = r; i < r + k; i++) {
                    for (int j = c; j < c + k; j++) {
                        values[idx++] = grid[i][j];
                    }
                }

                Arrays.sort(values);

                int minDiff = Integer.MAX_VALUE;

                // check adjacent after sort
                for (int i = 1; i < values.length; i++) {
                    if (values[i] == values[i - 1]) continue;
                    minDiff = Math.min(minDiff, values[i] - values[i - 1]);
                }

                result[r][c] = (minDiff == Integer.MAX_VALUE) ? 0 : minDiff;
            }
        }

        return result;
    }
}