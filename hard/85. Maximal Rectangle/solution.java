import java.util.Arrays;
import java.util.Stack;

class Solution {
    public int maximalRectangle(char[][] matrix) {
        if (matrix.length == 0 || matrix[0].length == 0) {
            return 0;
        }
        int[] heights = new int[matrix[0].length];
        int largest = 0;
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                int val = matrix[i][j] - '0';
                heights[j] = val == 0 ? 0 : val + heights[j];
            }
            largest = Math.max(largest, findmax(heights));
        }
        return largest;
    }

    public int findmax(int[] heights) {
        int n = heights.length;

        // next greater element index to the right of i
        int[] right_boundary = new int[n];
        Arrays.fill(right_boundary, n);
        Stack<Integer> stack = new Stack<>();
        for (int i = n - 1; i >= 0; i--) {
            while (!stack.isEmpty() && heights[stack.peek()] >= heights[i]) {
                stack.pop();
            }
            if (!stack.isEmpty()) {
                right_boundary[i] = stack.peek();
            }
            stack.push(i);
        }

        // next greater element index to the left of i
        int[] left_boundary = new int[n];
        Arrays.fill(left_boundary, -1);
        stack.clear();
        for (int i = 0; i < n; i++) {
            while (!stack.isEmpty() && heights[stack.peek()] >= heights[i]) {
                stack.pop();
            }
            if (!stack.isEmpty()) {
                left_boundary[i] = stack.peek();
            }
            stack.push(i);
        }

        int maxArea = 0;
        for (int i = 0; i < n; i++) {
            int width = right_boundary[i] - left_boundary[i] - 1;
            int area = width * heights[i];
            maxArea = Math.max(maxArea, area);
        }
        return maxArea;
    }
}