class Solution {
public:
    int largestAltitude(vector<int>& gain) {
        int current = 0;
        int maxAlt = 0;
        
        for (int g : gain) {
            current += g;
            maxAlt = max(maxAlt, current);
        }
        
        return maxAlt;
    }
};