class Solution {
public:
    int areaOfMaxDiagonal(vector<vector<int>>& dims) {
        int bestArea = 0;
        long long bestDiag = 0;

        for (auto &d : dims) {
            int w = d[0], h = d[1];

            long long diag = 1LL * w * w + 1LL * h * h; // compare squared diagonal

            if (diag > bestDiag || (diag == bestDiag && w * h > bestArea)) {
                bestDiag = diag;
                bestArea = w * h;
            }
        }

        return bestArea;
    }
};