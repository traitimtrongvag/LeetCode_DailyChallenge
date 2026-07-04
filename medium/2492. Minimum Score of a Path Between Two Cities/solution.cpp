class Solution {
public:
    int minScore(int n, vector<vector<int>>&roads) {
        vector<vector<pair<int,int>>>g(n+1);

        // build graph
        for(auto &r:roads){
            int u=r[0],v=r[1],w=r[2];
            g[u].push_back({v,w});
            g[v].push_back({u,w});
        }

        int ans=INT_MAX;
        vector<int>vis(n+1);

        // visit component containing city 1
        auto dfs=[&](this auto&&self,int u)->void{
            vis[u]=1;

            for(auto &[v,w]:g[u]){
                ans=min(ans,w);
                if(!vis[v])self(v);
            }
        };

        dfs(1);
        return ans;
    }
};