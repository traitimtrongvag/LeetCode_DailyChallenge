class Solution {
public:
    int maximumSafenessFactor(vector<vector<int>>&g) {
        int n=g.size();
        vector d(n,vector<int>(n,1e9));
        queue<pair<int,int>>q;

        // multi source bfs from thieves
        for(int i=0;i<n;i++)
            for(int j=0;j<n;j++)
                if(g[i][j])
                    d[i][j]=0,q.push({i,j});

        int dx[]={1,-1,0,0},dy[]={0,0,1,-1};

        // distance to nearest thief
        while(q.size()){
            auto [x,y]=q.front();
            q.pop();

            for(int k=0;k<4;k++){
                int nx=x+dx[k],ny=y+dy[k];
                if(nx<0||ny<0||nx>=n||ny>=n||d[nx][ny]!=1e9)continue;
                d[nx][ny]=d[x][y]+1;
                q.push({nx,ny});
            }
        }

        // maximin path (dijkstra)
        priority_queue<vector<int>>pq;
        vector vis(n,vector<int>(n));

        pq.push({d[0][0],0,0});

        while(pq.size()){
            auto t=pq.top();
            pq.pop();

            int s=t[0],x=t[1],y=t[2];

            if(vis[x][y])continue;
            vis[x][y]=1;

            if(x==n-1&&y==n-1)return s;

            for(int k=0;k<4;k++){
                int nx=x+dx[k],ny=y+dy[k];
                if(nx<0||ny<0||nx>=n||ny>=n||vis[nx][ny])continue;

                // path value = min safeness on path so far
                pq.push({min(s,d[nx][ny]),nx,ny});
            }
        }

        return 0;
    }
};