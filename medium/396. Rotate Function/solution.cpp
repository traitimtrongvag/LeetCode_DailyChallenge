class Solution {
public:
    int maxRotateFunction(vector<int>& a) {
        int n = a.size();

        long long sum = 0, cur = 0;

        // initial F(0)
        for (int i = 0; i < n; i++) {
            sum += a[i];
            cur += 1LL * i * a[i];
        }

        long long best = cur;

        // transition: F(k) from F(k-1)
        for (int i = n - 1; i > 0; i--) {
            cur = cur + sum - 1LL * n * a[i];
            best = max(best, cur);
        }

        return (int)best;
    }
};