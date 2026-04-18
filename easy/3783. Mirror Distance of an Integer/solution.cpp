class Solution {
private:
    int rev(int n) {
        int a = 0;
        while(n > 0) {
            a = a * 10 + (n % 10);
            n /= 10;
        }
        return a;
    }
public:
    int mirrorDistance(int n) {
        int m = rev(n);
        return abs(m - n);
    }
};