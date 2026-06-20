class Solution {
public:
    int maxBuilding(int n, vector<vector<int>>& r) {
        // Add implicit restriction at building 1
        r.push_back({1, 0});
        sort(r.begin(), r.end());
        
        // Forward pass: constraints from left
        for (int i = 1; i < r.size(); i++) {
            int dist = r[i][0] - r[i-1][0];
            r[i][1] = min(r[i][1], r[i-1][1] + dist);
        }
        
        // Backward pass: constraints from right
        for (int i = r.size()-2; i >= 0; i--) {
            int dist = r[i+1][0] - r[i][0];
            r[i][1] = min(r[i][1], r[i+1][1] + dist);
        }
        
        // Find max height between consecutive restrictions
        int ans = 0;
        for (int i = 1; i < r.size(); i++) {
            int d = r[i][0] - r[i-1][0];
            int h = abs(r[i][1] - r[i-1][1]);
            // Peak height between two restricted points
            ans = max(ans, max(r[i-1][1], r[i][1]) + (d - h) / 2);
        }
        
        // Handle after last restriction to end
        ans = max(ans, r.back()[1] + n - r.back()[0]);
        return ans;
    }
};