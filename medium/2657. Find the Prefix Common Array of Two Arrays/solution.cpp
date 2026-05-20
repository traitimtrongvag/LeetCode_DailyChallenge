/*
Time: O(n)
Space: O(n)

Idea: count seen numbers from both arrays, common when freq becomes 2
*/

class Solution {
public:
    vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        int n = A.size();

        vector<int> freq(n + 1);
        vector<int> ans(n);

        int common = 0;

        for (int i = 0; i < n; i++) {
            if (++freq[A[i]] == 2) {
                common++;
            }

            if (++freq[B[i]] == 2) {
                common++;
            }

            ans[i] = common;
        }

        return ans;
    }
};