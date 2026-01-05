class Solution {
    public long maxMatrixSum(int[][] matrix) {
        long sum = 0;
        int min = Integer.MAX_VALUE;
        int negatives = 0;

        for (int[] row : matrix) {
            for (int val : row) {
                int abs = Math.abs(val);
                sum += abs;
                min = Math.min(min, abs);
                if (val < 0) negatives++;
            }
        }

        return (negatives & 1) == 0 ? sum : sum - 2L * min;
    }
}