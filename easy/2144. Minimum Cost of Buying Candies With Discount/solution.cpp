/*
Time: O(n log n)
Space: O(1) excluding sort

Idea:
Sort in descending order.

For every 3 candies:
- pay for the 2 most expensive
- get the cheapest one for free
*/

class Solution {
public:
    int minimumCost(vector<int>& cost) {
        sort(cost.begin(), cost.end(), greater<int>());

        int totalCost = 0;

        for (int i = 0; i < cost.size(); i++) {
            // every third candy is free
            if ((i + 1) % 3 == 0) {
                continue;
            }

            totalCost += cost[i];
        }

        return totalCost;
    }
};