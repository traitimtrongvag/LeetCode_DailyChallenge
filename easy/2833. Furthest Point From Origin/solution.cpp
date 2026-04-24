class Solution {
public:
    int furthestDistanceFromOrigin(string s) {
        int l = 0, r = 0, u = 0;

        for (char c : s) {
            if (c == 'L') l++;
            else if (c == 'R') r++;
            else u++; // '_'
        }

        int diff = abs(l - r);
        return diff + u;
    }
};