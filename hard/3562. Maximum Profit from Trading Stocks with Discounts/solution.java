class Result {

    int[] dp0;
    int[] dp1;
    int size;

    Result(int[] dp0, int[] dp1, int size) {
        this.dp0 = dp0;
        this.dp1 = dp1;
        this.size = size;
    }
}

class Solution {

    public int maxProfit(
        int n,
        int[] present,
        int[] future,
        int[][] hierarchy,
        int budget
    ) {
        List<Integer>[] g = new ArrayList[n];
        for (int i = 0; i < n; i++) {
            g[i] = new ArrayList<>();
        }
        for (int[] e : hierarchy) {
            g[e[0] - 1].add(e[1] - 1);
        }

        return dfs(0, present, future, g, budget).dp0[budget];
    }

    private Result dfs(
        int u,
        int[] present,
        int[] future,
        List<Integer>[] g,
        int budget
    ) {
        int cost = present[u];
        int dCost = present[u] / 2;
        // dp[u][state][budget]
        // state = 0: Do not purchase parent node, state = 1: Must purchase parent node
        int[] dp0 = new int[budget + 1];
        int[] dp1 = new int[budget + 1];

        // subProfit[state][budget]
        // state = 0: discount not available, state = 1: discount available
        int[] subProfit0 = new int[budget + 1];
        int[] subProfit1 = new int[budget + 1];
        int uSize = cost;

        for (int v : g[u]) {
            Result childResult = dfs(v, present, future, g, budget);
            uSize += childResult.size;

            for (int i = budget; i >= 0; i--) {
                for (int sub = 0; sub <= Math.min(childResult.size, i); sub++) {
                    if (i - sub >= 0) {
                        subProfit0[i] = Math.max(
                            subProfit0[i],
                            subProfit0[i - sub] + childResult.dp0[sub]
                        );
                        subProfit1[i] = Math.max(
                            subProfit1[i],
                            subProfit1[i - sub] + childResult.dp1[sub]
                        );
                    }
                }
            }
        }

        for (int i = 0; i <= budget; i++) {
            dp0[i] = subProfit0[i];
            dp1[i] = subProfit0[i];
            if (i >= dCost) {
                dp1[i] = Math.max(
                    subProfit0[i],
                    subProfit1[i - dCost] + future[u] - dCost
                );
            }
            if (i >= cost) {
                dp0[i] = Math.max(
                    subProfit0[i],
                    subProfit1[i - cost] + future[u] - cost
                );
            }
        }

        return new Result(dp0, dp1, uSize);
    }
}