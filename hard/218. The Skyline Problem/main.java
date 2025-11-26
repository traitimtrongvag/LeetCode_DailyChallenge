// 218. The Skyline Problem
class Solution {
    public List<List<Integer>> getSkyline(int[][] buildings) {
        List<int[]> points = new ArrayList<>();
        for (int[] b : buildings) {
            points.add(new int[]{b[0], -b[2]});
            points.add(new int[]{b[1], b[2]});
        }
        points.sort((a, b) -> a[0] == b[0] ? a[1] - b[1] : a[0] - b[0]);
        
        TreeMap<Integer, Integer> heights = new TreeMap<>(Collections.reverseOrder());
        heights.put(0, 1);
        List<List<Integer>> result = new ArrayList<>();
        int prev = 0;
        
        for (int[] p : points) {
            int x = p[0], h = p[1];
            if (h < 0) {
                heights.put(-h, heights.getOrDefault(-h, 0) + 1);
            } else {
                heights.put(h, heights.get(h) - 1);
                if (heights.get(h) == 0) heights.remove(h);
            }
            
            int curr = heights.firstKey();
            if (curr != prev) {
                result.add(Arrays.asList(x, curr));
                prev = curr;
            }
        }
        return result;
    }
}