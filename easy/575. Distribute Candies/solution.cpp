class Solution {
public:
    int distributeCandies(vector<int>& candyType) {
        int n = candyType.size();
        int maxAllowed = n / 2; // sister can eat at most half of the candies
        
        // count distinct candy types
        unordered_set<int> types;
        for (int candy : candyType) {
            types.insert(candy);
        }
        
        int distinct = types.size();
        
        // sister can get at most min(distinct, maxAllowed)
        return min(distinct, maxAllowed);
    }
};