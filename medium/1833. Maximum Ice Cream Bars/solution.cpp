class Solution {
public:
    int maxIceCream(vector<int>& costs, int coins) {
        sort(costs.begin(), costs.end());
        
        int cnt = 0;
        for (int c : costs) {
            if (coins >= c) {
                coins -= c;
                cnt++;
            } else break;
        }
        return cnt;
    }
};