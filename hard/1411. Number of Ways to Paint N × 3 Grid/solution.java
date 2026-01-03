class Solution {
    public int numOfWays(int n) {

        final int mod = 1000000007;
        
        long pattern1 = 6;
        long pattern2 = 6;

        for(int i=2;i<=n;i++){
            long nextPattern1 = (pattern1*3 + pattern2*2) % mod;
            long nextPattern2 = (pattern1*2 + pattern2*2) % mod;
            pattern1 = nextPattern1;
            pattern2 = nextPattern2;
        }

        return (int)((pattern1+pattern2) % mod);
    }
}
