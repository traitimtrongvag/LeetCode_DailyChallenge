class Solution {
    private long f(int i,int j, int k ,int prices[], long dp[][][]){
        if(k < 0)return 0;
        if(i == prices.length){
            if(j != 0)return Long.MIN_VALUE / 2;
            return 0;
        }
        if(dp[i][j][k] != -1)return dp[i][j][k];
        long  buy = Long.MIN_VALUE, sell = Long.MIN_VALUE, skip = Long.MIN_VALUE;
        if(j == 0){
            buy = -prices[i] + f(i+1,1,k,prices,dp);
            sell = prices[i] + f(i+1,2,k,prices,dp);  
        }else if(j == 1){
            sell  = prices[i] + f(i+1,0,k-1,prices,dp);
        }else{
            buy   = -prices[i] + f(i+1,0,k-1,prices,dp);
        }
        skip = f(i+1,j,k,prices,dp);
        return dp[i][j][k] = Math.max(buy,Math.max(sell,skip)); 
    }
    public long maximumProfit(int[] prices, int k) {
        long[][][] dp = new long[prices.length][3][k];
        for(int i = 0;i < prices.length ; ++i){
            for(int j = 0 ; j < 3 ; ++j){
                Arrays.fill(dp[i][j],-1);
            }
        }
        return f(0,0,k-1,prices,dp);
    }
}