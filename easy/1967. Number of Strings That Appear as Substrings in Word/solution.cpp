class Solution {
public:
    int numOfStrings(vector<string>&p,string w) {
        int ans=0;
        for(auto&s:p)
            if(w.find(s)!=string::npos)
                ans++;
        return ans;
    }
};