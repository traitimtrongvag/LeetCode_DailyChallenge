class Solution {
    int[][] dir = {{1,0},{0,1},{-1,0},{0,-1}};
    public boolean check(int x, int y, int n, int m ) {
        return x>=0 && y>=0 && x<n && y<m;
    }
    public boolean dfs(int i, int j, int n, int m, int[][] grid) {
        if(i==n-1) return true;
        grid[i][j] = 2;
        boolean found = false;
        for(int k=0;k<4;k++) {
            int x = i + dir[k][0];
            int y = j + dir[k][1];
            if(check(x,y,n,m) && grid[x][y] == 0) {
                found = found || dfs(x,y,n,m,grid);
            }
        }
        return found;
    }
    public boolean possibe(int m, int row, int col, int[][] cells) {
        int[][] grid = new int[row][col];
        for(int i=0;i<m;i++)
            grid[cells[i][0]-1][cells[i][1]-1] = 1;
        
        for(int j=0;j<col;j++) {
            if(grid[0][j] == 0) {
                boolean pos = dfs(0,j,row,col,grid);
                if(pos)
                    return true;
            }
        }
        return false;
    }
    public int latestDayToCross(int row, int col, int[][] cells) {
        int days = 0;
        int l=1,r=cells.length;
        while(l<=r) {
            int m = l + (r-l)/2;
            if(possibe(m, row, col, cells)) {
                days = m;
                l = m+1;
            } else {
                r = m-1;
            }
        }
        return days;
    }
}