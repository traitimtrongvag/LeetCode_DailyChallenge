// Algorithm overview:
// Convert the array into a list for easy merging operations.
// Repeatedly merge the adjacent pair with the smallest sum.
// After each merge, check if the list becomes non-decreasing.
// Count how many merges are needed until the list is sorted.

class Solution {
    public int minimumPairRemoval(int[] nums) {
        // Convert input array to a list for dynamic resizing.
        List<Integer> list = new ArrayList<>();
        for (int i : nums) {
            list.add(i);
        }

        // Counts how many merge operations are performed.
        int count = 0;

        // Keep merging until the list becomes sorted.
        while (!sorted(list)) {
            int idx = 0;
            int minSum = Integer.MAX_VALUE;

            // Find the adjacent pair with the smallest sum.
            for (int i = 0; i < list.size() - 1; i++) {
                int sum = list.get(i) + list.get(i + 1);
                if (sum < minSum) {
                    minSum = sum;
                    idx = i;
                }
            }

            // Merge the selected pair into one element.
            int merged = list.get(idx) + list.get(idx + 1);
            list.set(idx, merged);
            list.remove(idx + 1);

            // One merge operation completed.
            count++;
        }

        // Return the total number of merges.
        return count;
    }

    // Check if the list is sorted in non-decreasing order.
    private boolean sorted(List<Integer> list) {
        for (int i = 1; i < list.size(); i++) {
            // If any element is smaller than the previous one, it's not sorted.
            if (list.get(i) < list.get(i - 1)) {
                return false;
            }
        }
        return true;
    }
}