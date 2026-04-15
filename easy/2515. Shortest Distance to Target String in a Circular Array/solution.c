int closestTarget(char** words, int wordsSize, char* target, int startIndex) {
    int ans = 1e9;

    for (int i = 0; i < wordsSize; i++) {
        if (strcmp(words[i], target) == 0) {
            int d = abs(i - startIndex);
            int cur = d < (wordsSize - d) ? d : (wordsSize - d);
            if (cur < ans) ans = cur;
        }
    }

    return ans == 1e9 ? -1 : ans;
}