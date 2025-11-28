// 56. Merge Intervals
int cmp(const void* a, const void* b) {
    int* arr1 = *(int**)a;
    int* arr2 = *(int**)b;
    return arr1[0] - arr2[0];
}

int** merge(int** intervals, int intervalsSize, int* intervalsColSize, int* returnSize, int** returnColumnSizes) {
    if (intervalsSize == 0) {
        *returnSize = 0;
        return NULL;
    }
    
    qsort(intervals, intervalsSize, sizeof(int*), cmp);
    
    int** merged = (int**)malloc(intervalsSize * sizeof(int*));
    *returnColumnSizes = (int*)malloc(intervalsSize * sizeof(int));
    *returnSize = 0;
    
    merged[0] = (int*)malloc(2 * sizeof(int));
    merged[0][0] = intervals[0][0];
    merged[0][1] = intervals[0][1];
    (*returnColumnSizes)[0] = 2;
    (*returnSize)++;
    
    for (int i = 1; i < intervalsSize; i++) {
        int* last = merged[*returnSize - 1];
        if (intervals[i][0] <= last[1]) {
            if (intervals[i][1] > last[1]) {
                last[1] = intervals[i][1];
            }
        } else {
            merged[*returnSize] = (int*)malloc(2 * sizeof(int));
            merged[*returnSize][0] = intervals[i][0];
            merged[*returnSize][1] = intervals[i][1];
            (*returnColumnSizes)[*returnSize] = 2;
            (*returnSize)++;
        }
    }
    
    return merged;
}