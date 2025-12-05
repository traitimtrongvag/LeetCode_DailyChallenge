char* convertToTitle(int columnNumber) {
    char* result = (char*)malloc(8 * sizeof(char));
    int index = 7;
    result[index] = '\0';
    
    while (columnNumber > 0) {
        columnNumber--;
        result[--index] = 'A' + columnNumber % 26;
        columnNumber /= 26;
    }
    
    return result + index;
}