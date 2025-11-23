// 2094. Finding 3-Digit Even Numbers
class Solution {
public:
    vector<int> findEvenNumbers(vector<int>& digits) {
        vector<int> count(10, 0);
        for (int d : digits) count[d]++;
        
        vector<int> result;
        for (int num = 100; num <= 998; num += 2) {
            vector<int> tempCount(10, 0);
            int n = num;
            while (n > 0) {
                tempCount[n % 10]++;
                n /= 10;
            }
            
            bool valid = true;
            for (int i = 0; i < 10; i++) {
                if (tempCount[i] > count[i]) {
                    valid = false;
                    break;
                }
            }
            if (valid) result.push_back(num);
        }
        return result;
    }
};