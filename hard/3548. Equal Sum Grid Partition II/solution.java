class Solution {
    int m, n;

    public boolean canPartitionGrid(int[][] grid) {
        m = grid.length;
        n = grid[0].length;

        long total = 0;
        for (int[] row : grid)
            for (int v : row)
                total += v;

        long sum = 0;
        HashSet<Long> halfSet = new HashSet<>();
        HashMap<Long, List<int[]>> map = new HashMap<>();

        // scan top -> bottom
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                long v = grid[i][j];
                sum += v;

                if (!map.containsKey(v)) {
                    map.put(v, new ArrayList<>());
                    if ((total + v) % 2 == 0)
                        halfSet.add((total + v) / 2);
                }
                map.get(v).add(new int[]{i, j});
            }

            if (sum * 2 == total) return true;
            if (halfSet.contains(sum)) {
                long need = sum * 2 - total;
                if (map.containsKey(need) && connectCheck(map.get(need), i, -1)) return true;
            }
        }

        sum = 0; map.clear(); halfSet.clear();

        // scan bottom -> top
        for (int i = m - 1; i >= 0; i--) {
            for (int j = 0; j < n; j++) {
                long v = grid[i][j];
                sum += v;

                if (!map.containsKey(v)) {
                    map.put(v, new ArrayList<>());
                    if ((total + v) % 2 == 0)
                        halfSet.add((total + v) / 2);
                }
                map.get(v).add(new int[]{i, j});
            }

            if (sum * 2 == total) return true;
            if (halfSet.contains(sum)) {
                long need = sum * 2 - total;
                if (map.containsKey(need) && connectCheck(map.get(need), i, -1)) return true;
            }
        }

        sum = 0; map.clear(); halfSet.clear();

        // scan left -> right
        for (int j = 0; j < n; j++) {
            for (int i = 0; i < m; i++) {
                long v = grid[i][j];
                sum += v;

                if (!map.containsKey(v)) {
                    map.put(v, new ArrayList<>());
                    if ((total + v) % 2 == 0)
                        halfSet.add((total + v) / 2);
                }
                map.get(v).add(new int[]{i, j});
            }

            if (sum * 2 == total) return true;
            if (halfSet.contains(sum)) {
                long need = sum * 2 - total;
                if (map.containsKey(need) && connectCheck(map.get(need), -1, j)) return true;
            }
        }

        sum = 0; map.clear(); halfSet.clear();

        // scan right -> left
        for (int j = n - 1; j >= 0; j--) {
            for (int i = 0; i < m; i++) {
                long v = grid[i][j];
                sum += v;

                if (!map.containsKey(v)) {
                    map.put(v, new ArrayList<>());
                    if ((total + v) % 2 == 0)
                        halfSet.add((total + v) / 2);
                }
                map.get(v).add(new int[]{i, j});
            }

            if (sum * 2 == total) return true;
            if (halfSet.contains(sum)) {
                long need = sum * 2 - total;
                if (map.containsKey(need) && connectCheck(map.get(need), -1, j)) return true;
            }
        }

        return false;
    }

    private boolean connectCheck(List<int[]> pos, int row, int col) {
        for (int[] p : pos) {
            boolean ok = true;

            if (row == 0 || row == m - 1) {
                if (p[1] != 0 && p[1] != n - 1) ok = false;
            } else if (col == 0 || col == n - 1) {
                if (p[0] != 0 && p[0] != m - 1) ok = false;
            }

            if (row > 0 && n == 1) {
                if (p[0] != 0 && p[0] != m - 1 && p[0] != row) ok = false;
            } else if (col > 0 && m == 1) {
                if (p[1] != 0 && p[1] != n - 1 && p[1] != col) ok = false;
            }

            if (ok) return true;
        }
        return false;
    }
}