/*
Time: O(n)
Space: O(n)
Beats: 100%
Idea: count freq, then check if counts are unique
*/

class Solution {
public:
    bool uniqueOccurrences(vector<int>& arr) {
        unordered_map<int, int> freq;

        // count occurrences
        for (int x : arr) {
            freq[x]++;
        }

        unordered_set<int> seen;

        // check uniqueness of counts
        for (auto &p : freq) {
            if (seen.count(p.second)) return false;
            seen.insert(p.second);
        }

        return true;
    }
};