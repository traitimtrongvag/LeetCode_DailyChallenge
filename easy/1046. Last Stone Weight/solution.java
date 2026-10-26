import java.util.Collections;
import java.util.PriorityQueue;

class Solution {
    public int lastStoneWeight(int[] stones) {
        PriorityQueue<Integer> heap = new PriorityQueue<>(Collections.reverseOrder());
        for (int stone : stones) {
            heap.add(stone);
        }

        while (heap.size() > 1) {
            int first = heap.remove();
            int second = heap.remove();
            if (first != second) {
                heap.add(first - second);
            }
        }

        return heap.isEmpty() ? 0 : heap.remove();
    }
}
