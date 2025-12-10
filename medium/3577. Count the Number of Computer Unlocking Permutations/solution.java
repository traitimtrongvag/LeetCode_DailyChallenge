class Solution {
    int mod = 1000000007;
    public int countPermutations(int[] complexity) {
        long ans = 1; int n = complexity.length;
        for(int i=1; i<n; i++){
            if(complexity[i] <= complexity[0]){
                return 0;
            }
        }
        for(int i=1; i<n; i++){
            ans = (ans * i) % mod;
        }
        return (int)ans;
    }
}