class Solution {
public:
    long long maxRunTime(int n, vector<int>& batteries) {
        long long left = 1, right = 0;
        for (int battery : batteries) {
            right += battery;
        }
        right /= n;
        
        while (left <= right) {
            long long mid = left + (right - left) / 2;
            long long total = 0;
            for (int battery : batteries) {
                total += min((long long)battery, mid);
            }
            if (total >= mid * n) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return right;
    }
};