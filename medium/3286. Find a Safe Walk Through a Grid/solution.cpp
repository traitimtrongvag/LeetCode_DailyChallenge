class Solution {
public:
    bool findSafeWalk(vector<vector<int>>&g,int h) {
        int m=g.size(),n=g[0].size();
        vector d(m,vector<int>(n,INT_MAX));
        deque<tuple<int,int,int>>q;

        d[0][0]=g[0][0];
        q.push_front({g[0][0],0,0});

        int dx[]={0,0,1,-1},dy[]={1,-1,0,0};

        while(q.size()){
            auto [w,x,y]=q.front();
            q.pop_front();

            if(w!=d[x][y])continue;

            for(int k=0;k<4;k++){
                int nx=x+dx[k],ny=y+dy[k];

                if(nx<0||nx>=m||ny<0||ny>=n)continue;
                if(w+g[nx][ny]>=d[nx][ny])continue;

                if(!g[nx][ny]){
                    d[nx][ny]=w;
                    q.push_front({w,nx,ny});
                }else{
                    d[nx][ny]=w+1;
                    q.push_back({w+1,nx,ny});
                }
            }
        }

        return d[m-1][n-1]<h;
    }
};