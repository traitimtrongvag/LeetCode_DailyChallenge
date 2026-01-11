class Solution {
    public int maximalRectangle(char[][] matrix) {
        int[] heights = new int[matrix[0].length];
        if(matrix[0].length == 0){
            return 0;
        }
        int largest = 0;
        for(int i = 0 ; i < matrix.length ; i++){
            for(int j = 0; j < matrix[0].length ; j++){
                int val = matrix[i][j] - '0';
                if(val == 0){
                    heights[j] = 0;
                }else{
                    heights[j] = val + heights[j];
                }
            }
            largest = Math.max(largest,findmax(heights));
        }
        return largest;
    }
    public int findmax(int[] heights){
        int[] left = new int[heights.length];
        Arrays.fill(left,heights.length);
        Stack<Integer> stack = new Stack<>();
        for(int i = heights.length - 1 ; i >= 0 ; i--){
            while(!stack.isEmpty() && heights[stack.peek()] >= heights[i]){
                stack.pop();
            }
            if(!stack.isEmpty()){
                left[i] = stack.peek();
            }
            stack.push(i);
        }
        int[] right = new int[heights.length];
        Arrays.fill(right,-1);
        Stack<Integer> stack1 = new Stack<>();
        for(int i = 0 ; i < heights.length ; i++){
            while(!stack1.isEmpty() && heights[stack1.peek()] >= heights[i]){
                stack1.pop();
            }
            if(!stack1.isEmpty()){
                right[i] = stack1.peek();
            }
            stack1.push(i);
        }
        int maxArea = 0;
        for(int i = 0 ; i < heights.length ; i++){
            int width = 1;
            int l = left[i] - i - 1;
            int r = i - right[i] - 1;
            int area = l + r + width;
            int mul = area * heights[i];
            maxArea = Math.max(maxArea,mul);
        }
        return maxArea;
    }
}