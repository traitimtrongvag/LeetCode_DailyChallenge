class Solution {
public:
    int mod = 1e9 + 7;

    int func(int i, int j, int k, int &limit,
             vector<vector<vector<int>>> &dp) {

        if(i < 0 || j < 0) return 0;

        if(i == 0 && j == 0) return 1;

        if(i == 0){
            if(j > 0 && j <= limit && k == 0) return 1;
            return 0;
        }

        if(j == 0){
            if(i > 0 && i <= limit && k == 1) return 1;
            return 0;
        }

        if(dp[i][j][k] != -1) return dp[i][j][k];

        long long ways = 0;

        // last group was 1 → place zeros
        if(k & 1){
            for(int x = 1; x <= min(i, limit); x++){
                ways = (ways + func(i - x, j, 0, limit, dp)) % mod;
            }
        }
        // last group was 0 → place ones
        else{
            for(int x = 1; x <= min(j, limit); x++){
                ways = (ways + func(i, j - x, 1, limit, dp)) % mod;
            }
        }

        return dp[i][j][k] = ways;
    }

    int numberOfStableArrays(int zero, int one, int limit) {

        vector<vector<vector<int>>> dp(
            zero + 1,
            vector<vector<int>>(one + 1, vector<int>(2, -1))
        );

        return (func(zero, one, 1, limit, dp) +
                func(zero, one, 0, limit, dp)) % mod;
    }
};