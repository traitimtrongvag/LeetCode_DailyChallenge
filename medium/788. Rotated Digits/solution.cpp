class Solution {
public:
    bool good(int x) {
        bool changed = false;

        while (x > 0) {
            int d = x % 10;
            x /= 10;

            // invalid digits
            if (d == 3 || d == 4 || d == 7) return false;

            // digits that change after rotate
            if (d == 2 || d == 5 || d == 6 || d == 9) {
                changed = true;
            }
        }

        return changed;
    }

    int rotatedDigits(int n) {
        int cnt = 0;

        for (int i = 1; i <= n; i++) {
            if (good(i)) cnt++;
        }

        return cnt;
    }
};