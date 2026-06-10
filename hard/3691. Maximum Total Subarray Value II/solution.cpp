struct SparseTable {
private:
    int n;
    vector<vector<int>> mn;
    vector<vector<int>> mx;      
    vector<int> logVal;          

public:

    SparseTable(const vector<int>& a) {
        n = a.size();
        int maxLog = 32 - __builtin_clz(n); 

        mn.assign(n, vector<int>(maxLog));
        mx.assign(n, vector<int>(maxLog));
        logVal.assign(n + 1, 0);


        for (int i = 2; i <= n; i++)
            logVal[i] = logVal[i / 2] + 1;


        for (int i = 0; i < n; i++) {
            mx[i][0] = a[i];
            mn[i][0] = a[i];
        }


        for (int j = 1; (1 << j) <= n; j++) {
            for (int i = 0; i + (1 << j) <= n; i++) {
                mn[i][j] = min(mn[i][j - 1], mn[i + (1 << (j - 1))][j - 1]);
            }
        }

        for (int j = 1; (1 << j) <= n; j++) {
            for (int i = 0; i + (1 << j) <= n; i++) {
                mx[i][j] = max(mx[i][j - 1], mx[i + (1 << (j - 1))][j - 1]);
            }
        }
    }

    int queryMin(int l, int r) {
        int j = logVal[r - l + 1];
        return min(mn[l][j], mn[r - (1 << j) + 1][j]);
    }

    int queryMax(int l, int r) {
        int j = logVal[r - l + 1];
        return max(mx[l][j], mx[r - (1 << j) + 1][j]);
    }
};

class Solution {
public:
    long long maxTotalValue(vector<int>& a, int k) {
        long long ans = 0;
        SparseTable st(a);
        int n = (int)a.size();
        multiset<pair<int,pair<int,int>>>s;
        for(int i = 0 ; i < n ; i++) {
            s.insert({st.queryMax(0,i) - st.queryMin(0,i),{0,i}});
        }
        while(k--) {
            int x = s.rbegin()->first;
            int l = s.rbegin()->second.first;
            int r = s.rbegin()->second.second;
            s.erase(s.find({x,{l,r}}));
            if(l+1 <= r) {
                s.insert({st.queryMax(l+1,r) - st.queryMin(l+1,r),{l+1,r}});
            }
            ans += x;
        }
        return ans;
    }
};