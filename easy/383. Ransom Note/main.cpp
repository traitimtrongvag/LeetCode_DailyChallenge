// 383. Ransom Note
#include <unordered_map>
#include <string>
using namespace std;

class Solution {
public:
    bool canConstruct(string ransomNote, string magazine) {
        unordered_map<char, int> count;
        
        for (char c : magazine) {
            count[c]++;
        }
        
        for (char c : ransomNote) {
            if (count[c] <= 0) {
                return false;
            }
            count[c]--;
        }
        
        return true;
    }
};