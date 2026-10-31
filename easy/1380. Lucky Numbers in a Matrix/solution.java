import java.util.ArrayList;
import java.util.List;

class Solution {
    public List<Integer> luckyNumbers(int[][] matrix) {
        int[] rowMin = new int[matrix.length];
        int[] colMax = new int[matrix[0].length];

        for (int i = 0; i < matrix.length; i++) {
            rowMin[i] = matrix[i][0];
            for (int j = 1; j < matrix[i].length; j++) {
                rowMin[i] = Math.min(rowMin[i], matrix[i][j]);
            }
        }

        for (int j = 0; j < matrix[0].length; j++) {
            colMax[j] = matrix[0][j];
            for (int i = 1; i < matrix.length; i++) {
                colMax[j] = Math.max(colMax[j], matrix[i][j]);
            }
        }

        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                if (matrix[i][j] == rowMin[i] && matrix[i][j] == colMax[j]) {
                    result.add(matrix[i][j]);
                }
            }
        }

        return result;
    }
}
