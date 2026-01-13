class Solution {
    public double separateSquares(int[][] squares) {
        double minY = 0.0;
        double totalArea = 0.0;
        double maxY = 0.0;
        for (int[] sq : squares) {
            int y = sq[1];
            int l = sq[2];
            double top = (double) y + (double) l;
            if (top > maxY) maxY = top;
            totalArea += (double) l * (double) l;
        }
        while (maxY - minY > 1e-5) {
            double m = (minY + maxY) / 2.0;
            double below = 0.0;
            for (int[] sq : squares) {
                int y = sq[1];
                int l = sq[2];
                if ((double) y < m) {
                    double cut = m - (double) y;
                    double h = Math.min(cut, (double) l);
                    below += (double) l * h;
                }
            }
            double above = totalArea - below;
            if (above - below > 0.0) {
                minY = m;
            } else {
                maxY = m;
            }
        }
        return minY;
    }
}