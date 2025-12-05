class Solution {
public:
    int sumOfEncryptedInt(vector<int>& nums) {
        int total = 0;
        
        for (int num : nums) {
            total += encrypt(num);
        }
        
        return total;
    }
    
private:
    int encrypt(int num) {
        if (num == 0) {
            return 0;
        }
        
        int maxDigit = 0;
        int count = 0;
        int n = num;
        
        // Find the maximum digit and count digits
        while (n > 0) {
            int digit = n % 10;
            if (digit > maxDigit) {
                maxDigit = digit;
            }
            count++;
            n /= 10;
        }
        
        // Build the encrypted number
        int result = 0;
        for (int i = 0; i < count; i++) {
            result = result * 10 + maxDigit;
        }
        
        return result;
    }
};