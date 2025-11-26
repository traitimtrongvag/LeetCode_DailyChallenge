// 34. Find First and Last Position of Element in Sorted Array
int* searchRange(int* nums, int numsSize, int target, int* returnSize) {
    int* result = (int*)malloc(2 * sizeof(int));
    *returnSize = 2;
    result[0] = -1;
    result[1] = -1;
    
    if (numsSize == 0) return result;
    
    int left = 0, right = numsSize - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] >= target) {
            if (nums[mid] == target) result[0] = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    
    left = 0, right = numsSize - 1;
    while (left <= right) {
        int mid = left + (right - left) / 2;
        if (nums[mid] <= target) {
            if (nums[mid] == target) result[1] = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    return result;
}