class Solution {
public:
    int countBeautifulPairs(vector<int>& nums) {
        int count = 0;
        int n = nums.size();
        
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int first = firstDigit(nums[i]);
                int last = nums[j] % 10;
                
                if (gcd(first, last) == 1) {
                    count++;
                }
            }
        }
        
        return count;
    }
    
private:
    int firstDigit(int num) {
        while (num >= 10) {
            num /= 10;
        }
        return num;
    }
    
    int gcd(int a, int b) {
        if (b == 0) return a;
        return gcd(b, a % b);
    }
};