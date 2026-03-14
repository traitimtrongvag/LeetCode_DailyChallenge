char* getHappyString(int n, int k) {
    int max = 1;
    for (int i = 1; i < n; i++) max *= 2;
    max *= 3;

    char **list = (char**)malloc(sizeof(char*) * max);
    int size = 0;

    char cur[20];

    void dfs(int pos) {
        if (pos == n) {
            cur[pos] = '\0';
            list[size] = (char*)malloc(n + 1);
            strcpy(list[size], cur);
            size++;
            return;
        }

        char letters[3] = {'a','b','c'};
        for (int i = 0; i < 3; i++) {
            if (pos > 0 && cur[pos-1] == letters[i]) continue;
            cur[pos] = letters[i];
            dfs(pos + 1);
        }
    }

    dfs(0);

    if (k > size) {
        char *empty = (char*)malloc(1);
        empty[0] = '\0';
        return empty;
    }

    char *ans = (char*)malloc(n + 1);
    strcpy(ans, list[k - 1]);
    return ans;
}