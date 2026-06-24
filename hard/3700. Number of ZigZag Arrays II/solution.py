class Solution:
    def zigZagArrays(self, n: int, l: int, r: int) -> int:
        mod = 10**9 + 7
        m = r - l + 1

        def mul(a, b):
            c = [[0] * len(b[0]) for _ in range(len(a))]
            for i in range(len(a)):
                for k in range(len(b)):
                    if a[i][k]:
                        for j in range(len(b[0])):
                            c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % mod
            return c

        # transition matrix
        # can go from x -> y if x + y > m + 1
        a = [[i + j >= m for j in range(m)] for i in range(m)]

        # identity matrix
        p = [[i == j for j in range(m)] for i in range(m)]

        # matrix exponentiation: A^(n-2)
        e = n - 2
        while e:
            if e & 1:
                p = mul(p, a)
            a = mul(a, a)
            e >>= 1

        # F2[i] = number of valid arrays of length 2 ending at i
        return sum(x[0] for x in mul(p, [[i] for i in range(m)])) * 2 % mod