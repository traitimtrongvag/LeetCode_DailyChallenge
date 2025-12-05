class Solution {
    bool isPrime(int num) {
        if (num <= 1) return false;
        if (num == 2) return true;
        if (num % 2 == 0) return false;
        for (int i = 3; i * i <= num; i += 2) {
            if (num % i == 0) return false;
        }
        return true;
    }

public:
    int mostFrequentPrime(vector<vector<int>>& mat) {
        int m = mat.size(), n = mat[0].size();
        unordered_map<int, int> freq;
        
        vector<pair<int, int>> dirs = {
            {0,1}, {1,0}, {0,-1}, {-1,0},
            {1,1}, {1,-1}, {-1,1}, {-1,-1}
        };
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                for (auto [dx, dy] : dirs) {
                    int x = i, y = j;
                    int num = 0;
                    while (x >= 0 && x < m && y >= 0 && y < n) {
                        num = num * 10 + mat[x][y];
                        if (num > 10 && isPrime(num)) {
                            freq[num]++;
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
        
        int maxFreq = 0, result = -1;
        for (auto [num, count] : freq) {
            if (count > maxFreq || (count == maxFreq && num > result)) {
                maxFreq = count;
                result = num;
            }
        }
        return result;
    }
};