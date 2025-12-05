typedef struct {
    int x;
    int h;
} Point;

int cmp(const void* a, const void* b) {
    Point* p1 = (Point*)a;
    Point* p2 = (Point*)b;
    if (p1->x != p2->x) return p1->x - p2->x;
    return p1->h - p2->h;
}

int** getSkyline(int** buildings, int buildingsSize, int* buildingsColSize, int* returnSize, int** returnColumnSizes) {
    Point* points = malloc(2 * buildingsSize * sizeof(Point));
    int idx = 0;
    for (int i = 0; i < buildingsSize; i++) {
        points[idx].x = buildings[i][0];
        points[idx++].h = -buildings[i][2];
        points[idx].x = buildings[i][1];
        points[idx++].h = buildings[i][2];
    }
    qsort(points, 2 * buildingsSize, sizeof(Point), cmp);
    
    int* heights = malloc(40000 * sizeof(int));
    int heapSize = 0;
    heights[heapSize++] = 0;
    
    int** result = malloc(2 * buildingsSize * sizeof(int*));
    *returnColumnSizes = malloc(2 * buildingsSize * sizeof(int));
    *returnSize = 0;
    int prev = 0;
    
    for (int i = 0; i < 2 * buildingsSize; i++) {
        int x = points[i].x, h = points[i].h;
        if (h < 0) {
            heights[heapSize++] = -h;
            for (int j = heapSize - 1; j > 0 && heights[j] > heights[j - 1]; j--) {
                int temp = heights[j];
                heights[j] = heights[j - 1];
                heights[j - 1] = temp;
            }
        } else {
            int pos = 0;
            while (pos < heapSize && heights[pos] != h) pos++;
            for (int j = pos; j < heapSize - 1; j++) {
                heights[j] = heights[j + 1];
            }
            heapSize--;
        }
        
        int curr = heights[0];
        if (curr != prev) {
            result[*returnSize] = malloc(2 * sizeof(int));
            result[*returnSize][0] = x;
            result[*returnSize][1] = curr;
            (*returnColumnSizes)[*returnSize] = 2;
            (*returnSize)++;
            prev = curr;
        }
    }
    free(points);
    free(heights);
    return result;
}