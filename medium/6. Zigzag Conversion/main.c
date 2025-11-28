// 6. Zigzag Conversion
char* convert(char* s, int numRows) {
    if (numRows == 1) return s;
    
    int len = strlen(s);
    if (len <= numRows) return s;
    
    char** rows = (char**)malloc(numRows * sizeof(char*));
    for (int i = 0; i < numRows; i++) {
        rows[i] = (char*)malloc((len + 1) * sizeof(char));
        rows[i][0] = '\0';
    }
    
    int curRow = 0;
    int goingDown = 0; // 0 for down, 1 for up
    
    for (int i = 0; i < len; i++) {
        strncat(rows[curRow], &s[i], 1);
        
        if (curRow == 0 || curRow == numRows - 1) {
            goingDown = !goingDown;
        }
        curRow += goingDown ? 1 : -1;
    }
    
    char* result = (char*)malloc((len + 1) * sizeof(char));
    result[0] = '\0';
    
    for (int i = 0; i < numRows; i++) {
        strcat(result, rows[i]);
        free(rows[i]);
    }
    free(rows);
    
    return result;
}