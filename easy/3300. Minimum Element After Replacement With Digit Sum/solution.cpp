/*
Time: O(n * digits)
Space: O(1)

Idea:
Replace every number with its digit sum,
then return the minimum value.
*/

class Solution {
public:
    int getDigitSum(int number) {
        int digitSum = 0;

        // calculate sum of digits
        while (number > 0) {
            digitSum += number % 10;
            number /= 10;
        }

        return digitSum;
    }

    int minElement(vector<int>& nums) {
        int minimumValue = INT_MAX;

        for (int number : nums) {
            int digitSum = getDigitSum(number);

            minimumValue = min(minimumValue, digitSum);
        }

        return minimumValue;
    }
};