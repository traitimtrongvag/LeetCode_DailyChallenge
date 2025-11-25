// 11. Container With Most Water
int maxArea(int* height, int heightSize) {
    int left = 0, right = heightSize - 1;
    int maxArea = 0;
    while (left < right) {
        int area = fmin(height[left], height[right]) * (right - left);
        if (area > maxArea) maxArea = area;
        if (height[left] < height[right]) {
            left++;
        } else {
            right--;
        }
    }
    return maxArea;
}