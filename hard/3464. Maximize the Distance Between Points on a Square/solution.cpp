class Solution {
public:
    int maxDistance(int s, vector<vector<int>>& pts, int k) {
        int n = pts.size();
        long long perim = 4LL * s;

        vector<long long> d(n);
        vector<int> nxt(n);

        // map points to perimeter distance
        for (int i = 0; i < n; i++) {
            long long x = pts[i][0], y = pts[i][1];
            if (x == 0 || y == s) d[i] = perim - x - y;
            else d[i] = x + y;
        }

        sort(d.begin(), d.end());

        // check if distance >= x is possible
        auto ok = [&](long long x) {
            for (int i = 0, j = 0; i < n; i++) {
                j = max(j, i + 1);
                while (j < n && d[j] < d[i] + x) j++;
                nxt[i] = j;
            }

            long long limit = perim - x;

            for (int i = 0; i < n; i++) {
                int j = i;

                // jump k-1 times
                for (int t = k - 1; t > 0 && j < n; t--) {
                    j = nxt[j];
                }

                if (j < n && d[j] - d[i] <= limit) {
                    return true;
                }
            }

            return false;
        };

        long long lo = 1, hi = perim / k;

        // binary search answer
        while (lo < hi) {
            long long mid = (lo + hi + 1) / 2;
            if (ok(mid)) lo = mid;
            else hi = mid - 1;
        }

        return (int)hi;
    }
};