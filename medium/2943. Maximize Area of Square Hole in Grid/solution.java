class Solution {
    public int maximizeSquareHoleArea(int n, int m, int[] hBars, int[] vBars)
    {
        // sort both lists of removable bars
        Arrays.sort(hBars);
        Arrays.sort(vBars);

        // find the length of longest consecutive horizontal removable bars
        int lch = 0, counter = 0, prev = -1;
        for (int i: hBars) {
            if (i == prev + 1) {
                counter++;
            }
            else {
                lch = Math.max(lch, counter);
                counter = 1;
            }
            prev = i;
        }
        lch = Math.max(lch, counter);
        
        
        // find the length of longest consecutive vertical removable bars
        int lcv = 0;
        counter = 0; prev = -1;
        for (int i: vBars) {
            if (i == prev + 1) {
                counter++;
            }
            else {
                lcv = Math.max(lcv, counter);
                counter = 1;
            }
            prev = i;
        }
        lcv = Math.max(lcv, counter);

        // Side of the biggest square : 
        int s = Math.min(lch, lcv) + 1;
         
        // return largest square's area : 
        return s*s;
    }
}