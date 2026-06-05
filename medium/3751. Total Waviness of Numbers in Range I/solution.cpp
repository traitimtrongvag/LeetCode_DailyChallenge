class Solution {
public:
    // turs all digit in the number in a vector
    vector<int> numbers(int n) {

        vector<int> v;

        string s = to_string(n);

        for (char c : s) {
            v.push_back(c - '0');
        }
        return v;
    }

    int waviness(vector<int> nums) {
        int n = nums.size();
        int cnt=0;
        for (int i = 1; i < n - 1; i++) {
            if ((nums[i - 1] > nums[i] && nums[i] > nums[i + 1]) ||
                (nums[i - 1] < nums[i] && nums[i] < nums[i + 1]) ||
                (nums[i - 1] == nums[i] || nums[i] == nums[i + 1])) {
                continue;
            }
            else cnt++;
        }
        return cnt;
    }

    int totalWaviness(int num1, int num2) {
        int i = num1;
        int waves = 0;
        while (i <= num2) {
            if ((i % 100) == i) {
                i++;
                continue;
            }
            vector<int>num=numbers(i);
            int check=waviness(num);
    
            if(check!=0) waves+=check;
            i++;
        }
        return waves;
    }
};