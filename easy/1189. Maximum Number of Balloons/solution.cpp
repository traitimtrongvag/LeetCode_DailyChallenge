class Solution {
public:
    int maxNumberOfBalloons(string text) {
        // Count frequency of each character in the input text
        vector<int> freq(26, 0);
        for (char c : text) {
            freq[c - 'a']++;
        }
        
        // The word "balloon" requires:
        // b: 1, a: 1, l: 2, o: 2, n: 1
        
        // Get counts for each required letter
        int b = freq['b' - 'a'];
        int a = freq['a' - 'a'];
        int l = freq['l' - 'a'] / 2;  // Need 2 l's per word
        int o = freq['o' - 'a'] / 2;  // Need 2 o's per word
        int n = freq['n' - 'a'];
        
        // The maximum number of balloons is limited by the letter with minimum count
        int result = min({b, a, l, o, n});
        
        return result;
    }
};