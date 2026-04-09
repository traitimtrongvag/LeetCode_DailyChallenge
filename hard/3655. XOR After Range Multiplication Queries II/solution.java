class Solution {
    public int xorAfterQueries(int[] a, int[][] qs) {
        int n = a.length, B = 316;
        long mod = 1_000_000_007;

        // handle small step (<= sqrt)
        for (int k = 1; k < B; k++) {
            long[] f = new long[n];
            boolean used = false;

            // init multiplicative array = 1
            for (int i = 0; i < n; i++) f[i] = 1;

            for (int[] q : qs) {
                if (q[2] != k) continue;
                used = true;

                int l = q[0], r = q[1];
                long v = q[3];

                f[l] = f[l] * v % mod;

                int last = l + (r - l) / k * k;
                if (last + k < n) {
                    f[last + k] = f[last + k] * inv(v, mod) % mod;
                }
            }

            if (!used) continue;

            // prefix accumulation with step k
            for (int i = 0; i < n; i++) {
                if (i >= k) f[i] = f[i] * f[i - k] % mod;
                if (f[i] != 1) {
                    a[i] = (int) (a[i] * f[i] % mod);
                }
            }
        }

        // handle large step (brute force)
        for (int[] q : qs) {
            if (q[2] < B) continue;

            for (int i = q[0]; i <= q[1]; i += q[2]) {
                a[i] = (int) ((long) a[i] * q[3] % mod);
            }
        }

        // compute final xor
        int res = 0;
        for (int x : a) res ^= x;
        return res;
    }

    // modular inverse (Fermat's little theorem)
    private long inv(long a, long m) {
        long res = 1, p = m - 2;
        while (p > 0) {
            if ((p & 1) == 1) res = res * a % m;
            a = a * a % m;
            p >>= 1;
        }
        return res;
    }
}