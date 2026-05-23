/*
Time: O(n log n)
Space: O(1)

Idea: sort by (minimum - actual) descending, do harder tasks first
*/

class Solution {
public:
    int minimumEffort(vector<vector<int>>& tasks) {
        sort(tasks.begin(), tasks.end(),
            [](vector<int>& a, vector<int>& b) {
                return (a[1] - a[0]) > (b[1] - b[0]);
            });

        int energy = 0;
        int cur = 0;

        for (auto& t : tasks) {
            int actual = t[0];
            int minimum = t[1];

            // raise initial energy if current is not enough
            if (cur < minimum) {
                energy += minimum - cur;
                cur = minimum;
            }

            cur -= actual;
        }

        return energy;
    }
};