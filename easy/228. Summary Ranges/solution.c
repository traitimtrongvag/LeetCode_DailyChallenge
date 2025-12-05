char** summaryRanges(int* nums, int numsSize, int* returnSize) {
    if (numsSize == 0) {
        *returnSize = 0;
        return NULL;
    }
    
    char** result = (char**)malloc(numsSize * sizeof(char*));
    *returnSize = 0;
    
    int start = nums[0];
    for (int i = 1; i <= numsSize; i++) {
        if (i < numsSize && nums[i] == nums[i - 1] + 1) {
            continue;
        } else {
            if (start == nums[i - 1]) {
                result[*returnSize] = (char*)malloc(12 * sizeof(char));
                sprintf(result[*returnSize], "%d", start);
            } else {
                result[*returnSize] = (char*)malloc(25 * sizeof(char));
                sprintf(result[*returnSize], "%d->%d", start, nums[i - 1]);
            }
            (*returnSize)++;
            if (i < numsSize) start = nums[i];
        }
    }
    
    return result;
}