// idea: https://youtu.be/YacKxgYqJ6o?si=2QhBLY7hSpwD5869
class Solution {
    public int mostBooked(int n, int[][] meetings) {
        PriorityQueue<Integer> avai_rooms = new PriorityQueue<>();
        for (int i = 0; i < n; i++) avai_rooms.add(i);

        PriorityQueue<long[]> used_rooms = new PriorityQueue<>(
                (a, b) -> a[0] == b[0] ? Long.compare(a[1], b[1]) : Long.compare(a[0], b[0])
        );

        int[] count = new int[n];
        Arrays.sort(meetings, Comparator.comparingInt(a -> a[0]));

        for (int[] m : meetings) {
            long start = m[0];
            long end = m[1];

            while (!used_rooms.isEmpty() && used_rooms.peek()[0] <= start) {
                long[] r = used_rooms.poll();
                avai_rooms.add((int) r[1]);
            }

            int avai_room;
            if (!avai_rooms.isEmpty()) {
                avai_room = avai_rooms.poll();
                used_rooms.add(new long[]{end, avai_room});
            } else {
                long[] r = used_rooms.poll();
                long finish_time = r[0];
                avai_room = (int) r[1];
                used_rooms.add(new long[]{finish_time + (end - start), avai_room});
            }
            count[avai_room]++;
        }

        int cnt = count[0];
        int ret_val = 0;
        for (int i = 1; i < n; i++) {
            if (count[i] > cnt) {
                cnt = count[i];
                ret_val = i;
            }
        }
        return ret_val;
    }
}