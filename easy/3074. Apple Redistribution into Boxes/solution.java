class Solution {
    public int minimumBoxes(int[] apple, int[] capacity) {
        int totalApples = 0;
        for (int e : apple) {
            totalApples += e;
        }

        Arrays.sort(capacity);

        int totalCapacity = capacity[capacity.length - 1];
        int box = 1;

        for (int i = capacity.length - 2; i >= 0; i--) {
            if (totalCapacity >= totalApples) {
                return box;
            }
            totalCapacity += capacity[i];
            box++;
        }

        return box;
    }
}
