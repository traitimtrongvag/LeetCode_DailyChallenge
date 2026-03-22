/*
Time: O(n)
Space: O(1)

Idea: moving between same parity costs 0, different parity costs 1
*/

class Solution {
public:
    int minCostToMoveChips(vector<int>& position) {
        int odd = 0;
        int even = 0;

        for (int x : position) {
            if (x % 2) {
                odd++;
            } else {
                even++;
            }
        }

        return min(odd, even);
    }
};