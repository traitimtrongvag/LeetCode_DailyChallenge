char* findTheString(int** lcp, int lcpSize, int* lcpColSize) {
    int n = lcpSize;

    char* res = (char*)malloc((n + 1) * sizeof(char));
    memset(res, 0, n + 1);

    char ch = 'a';

    // build string
    for (int i = 0; i < n; i++) {
        if (res[i] == 0) {

            if (ch > 'z') {
                res[0] = '\0';
                return res;
            }

            res[i] = ch;

            // fill same char if lcp > 0
            for (int j = i + 1; j < n; j++) {
                if (lcp[i][j] > 0) {
                    res[j] = ch;
                }
            }

            ch++;
        }
    }

    // validate
    for (int i = n - 1; i >= 0; i--) {
        for (int j = n - 1; j >= 0; j--) {

            if (res[i] != res[j]) {
                if (lcp[i][j] != 0) {
                    res[0] = '\0';
                    return res;
                }
            } else {
                int expected;

                // edge case: last row/col
                if (i == n - 1 || j == n - 1) {
                    expected = 1;
                } else {
                    expected = lcp[i + 1][j + 1] + 1;
                }

                if (lcp[i][j] != expected) {
                    res[0] = '\0';
                    return res;
                }
            }
        }
    }

    return res;
}