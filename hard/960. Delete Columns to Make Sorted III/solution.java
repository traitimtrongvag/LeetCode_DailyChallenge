class Solution {
    public int minDeletionSize(String[] s) {
        int a=1;
        int n=s[0].length();
        char[][] c = new char[s.length][n];
        for(int i=0; i<s.length; i++)c[i]=s[i].toCharArray();
        int[] d = new int[n];
        d[0]=1;
        for(int i=1; i<n; i++){
            d[i]=1;
            for(int j=i-1; j>=0; j--){
                if(d[j]<d[i])continue;
                if(sorted(c, j, i))d[i]=d[j]+1;
            }
            if(a<d[i])a=d[i];
        }
        return n-a;
    }
    private boolean sorted(char[][] c, int j, int i){
        for(char[] d:c){
            if(d[j]>d[i])return false;
        }
        return true;
    }
}