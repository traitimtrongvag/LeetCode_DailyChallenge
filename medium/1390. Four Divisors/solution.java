class Solution {
    public int sumFourDivisors(int[] nums) {
        int totalSum = 0;
        for (int value : nums) {
            int divisorSum = computeFourDivisorSum(value);
            if (divisorSum != 0) {
                totalSum += divisorSum;
            }
        }
        return totalSum;
    }
    
    private int computeFourDivisorSum(int number) {
        int root = (int) Math.round(Math.cbrt(number));
        if ((long) root * root * root == number && checkPrime(root)) {
            return 1 + root + root * root + number;
        }
        for (int factor = 2; factor * factor <= number; factor++) {
            if (number % factor == 0) {
                int other = number / factor;
                if (factor != other && checkPrime(factor) && checkPrime(other)) {
                    return 1 + factor + other + number;
                }
                return 0;
            }
        }
        return 0;
    }
    
    private boolean checkPrime(int num) {
        if (num < 2) return false;
        for (int div = 2; div * div <= num; div++) {
            if (num % div == 0) return false;
        }
        return true;
    }
}