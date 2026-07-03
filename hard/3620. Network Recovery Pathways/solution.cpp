class Solution {
public:
    int findMaxPathScore(vector<vector<int>>&e,vector<bool>&on,long long k) {
        int n=on.size();
        vector<vector<pair<int,int>>>g(n);

        int l=INT_MAX,r=0;

        // build graph using online nodes only
        for(auto &x:e){
            int u=x[0],v=x[1],w=x[2];
            if(!on[u]||!on[v])continue;

            g[u].push_back({v,w});
            l=min(l,w);
            r=max(r,w);
        }

        // can we keep all edges >= x and still reach with cost <= k ?
        auto ok=[&](int x){
            vector<long long>dp(n,-1);

            // min path cost from u -> n-1
            auto dfs=[&](this auto&&self,int u)->long long{
                if(u==n-1)return 0;
                if(dp[u]!=-1)return dp[u];

                long long res=LLONG_MAX/2;

                for(auto &[v,w]:g[u])
                    if(w>=x) // only use valid edges
                        res=min(res,self(v)+w);

                return dp[u]=res;
            };

            return dfs(0)<=k;
        };

        if(!ok(l))return -1;

        // binary search answer
        while(l<=r){
            int m=l+r>>1;

            if(ok(m))l=m+1;
            else r=m-1;
        }

        return r;
    }
};