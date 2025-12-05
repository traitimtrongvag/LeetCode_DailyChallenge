char* longestCommonPrefix(char** strs, int strsSize) {
    if (strsSize == 0) return "";
    
    for (int i = 0; strs[0][i]; i++) {
        char c = strs[0][i];
        for (int j = 1; j < strsSize; j++) {
            if (strs[j][i] == '\0' || strs[j][i] != c) {
                char* result = malloc(i + 1);
                strncpy(result, strs[0], i);
                result[i] = '\0';
                return result;
            }
        }
    }
    
    return strdup(strs[0]);
}