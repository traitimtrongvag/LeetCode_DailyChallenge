class Solution {
    public long maxProfit(int[] prices, int[] strategy, int k) {
        int n = prices.length, h = k >> 1;
        long base = 0, prev = 0, nxt = 0, best = 0;

        for (int i = 0; i < n; i++) base += (long) strategy[i] * prices[i];

        for (int i = 0; i < k; i++) {
            prev += (long) strategy[i] * prices[i];
            if (i >= h) nxt += prices[i];
        }

        best = Math.max(0, nxt - prev);

        for (int r = k; r < n; r++) {
            int l = r - k + 1;
            prev += (long) strategy[r] * prices[r]
                 - (long) strategy[l - 1] * prices[l - 1];
            nxt += prices[r] - prices[l - 1 + h];
            best = Math.max(best, nxt - prev);
        }

        return base + best;
    }
}