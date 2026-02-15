#include <stdio.h>
#include <stdlib.h>

/*
    Find the town judge.

    Idea:
    - The judge trusts nobody  -> outdegree = 0
    - Everybody else trusts the judge -> indegree = n - 1
    - We count indegree and outdegree for each person.
*/

int findJudge(int n, int** trust, int trustSize, int* trustColSize) {
    // Allocate memory for indegree and outdegree arrays
    int* indeg = (int*)calloc(n, sizeof(int));
    int* outdeg = (int*)calloc(n, sizeof(int));

    // Process trust relationships
    for (int i = 0; i < trustSize; i++) {
        int a = trust[i][0] - 1;  // convert to 0-based index
        int b = trust[i][1] - 1;

        outdeg[a]++;  // a trusts someone
        indeg[b]++;   // b is trusted by someone
    }

    // Check judge conditions
    for (int i = 0; i < n; i++) {
        if (indeg[i] == n - 1 && outdeg[i] == 0) {
            free(indeg);
            free(outdeg);
            return i + 1;  // convert back to 1-based index
        }
    }

    free(indeg);
    free(outdeg);
    return -1;  // no judge found
}