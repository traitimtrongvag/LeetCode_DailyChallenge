class Solution {
public:
    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        vector<pair<int, int>> points;
        for (auto& b : buildings) {
            points.push_back({b[0], -b[2]});
            points.push_back({b[1], b[2]});
        }
        sort(points.begin(), points.end());
        
        multiset<int> heights = {0};
        vector<vector<int>> result;
        int prev = 0;
        
        for (auto& p : points) {
            int x = p.first, h = p.second;
            if (h < 0) {
                heights.insert(-h);
            } else {
                heights.erase(heights.find(h));
            }
            
            int curr = *heights.rbegin();
            if (curr != prev) {
                result.push_back({x, curr});
                prev = curr;
            }
        }
        return result;
    }
};