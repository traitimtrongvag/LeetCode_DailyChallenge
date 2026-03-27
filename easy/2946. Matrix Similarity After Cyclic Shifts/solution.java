class Solution {
    public boolean areSimilar(int[][] mat, int k) {
        int rows = mat.length;
        int cols = mat[0].length;

        // normalize k to avoid extra cycles
        k %= cols;

        for (int i = 0; i < rows; i++) {

            // even row -> shift left
            if (i % 2 == 0) {
                for (int j = 0; j < cols; j++) {
                    int newCol = (j + k) % cols;

                    // check if after shift still matches original
                    if (mat[i][j] != mat[i][newCol]) {
                        return false;
                    }
                }
            } 
            // odd row -> shift right
            else {
                for (int j = 0; j < cols; j++) {
                    int newCol = (j - k + cols) % cols;

                    if (mat[i][j] != mat[i][newCol]) {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}