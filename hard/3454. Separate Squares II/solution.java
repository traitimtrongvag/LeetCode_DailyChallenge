class Solution {
    static class Event {
        long y;
        int event_type;
        long x_left, x_right;
        Event(long y, int event_type, long x_left, long x_right) {
            this.y = y; this.event_type = event_type; this.x_left = x_left; this.x_right = x_right;
        }
    }

    static class Slice {
        long start_y, height, width;
        Slice(long start_y, long height, long width) {
            this.start_y = start_y; this.height = height; this.width = width;
        }
    }

    private long union_width(ArrayList<long[]> intervals) {
        intervals.sort((a, b) -> {
            if (a[0] != b[0]) return Long.compare(a[0], b[0]);
            return Long.compare(a[1], b[1]);
        });

        long total_width = 0;
        long rightmost = (long)-1e18;

        for (long[] it : intervals) {
            long left = it[0], right = it[1];
            if (left > rightmost) {
                total_width += right - left;
                rightmost = right;
            } else if (right > rightmost) {
                total_width += right - rightmost;
                rightmost = right;
            }
        }
        return total_width;
    }

    public double separateSquares(int[][] squares) {
        ArrayList<Event> events = new ArrayList<>();

        for (int[] sq : squares) {
            long x = sq[0], y = sq[1], length = sq[2];
            events.add(new Event(y, 1, x, x + length));
            events.add(new Event(y + length, -1, x, x + length));
        }

        events.sort((a, b) -> {
            if (a.y != b.y) return Long.compare(a.y, b.y);
            if (a.event_type != b.event_type) return Integer.compare(a.event_type, b.event_type);
            if (a.x_left != b.x_left) return Long.compare(a.x_left, b.x_left);
            return Long.compare(a.x_right, b.x_right);
        });

        ArrayList<long[]> active_intervals = new ArrayList<>();
        long prev_y = events.get(0).y;
        long total_area = 0;
        ArrayList<Slice> horizontal_slices = new ArrayList<>();

        for (Event e : events) {
            long y = e.y;
            int event_type = e.event_type;
            long x_left = e.x_left;
            long x_right = e.x_right;

            if (y > prev_y && !active_intervals.isEmpty()) {
                long height = y - prev_y;

                ArrayList<long[]> tmp = new ArrayList<>(active_intervals.size());
                for (long[] it : active_intervals) tmp.add(new long[]{it[0], it[1]});
                long width = union_width(tmp);

                horizontal_slices.add(new Slice(prev_y, height, width));
                total_area += height * width;
            }

            if (event_type == 1) {
                active_intervals.add(new long[]{x_left, x_right});
            } else {
                for (int i = 0; i < active_intervals.size(); i++) {
                    long[] it = active_intervals.get(i);
                    if (it[0] == x_left && it[1] == x_right) {
                        active_intervals.remove(i);
                        break;
                    }
                }
            }

            prev_y = y;
        }

        double half = total_area / 2.0;
        double accumulated = 0.0;

        for (Slice sl : horizontal_slices) {
            long start_y = sl.start_y;
            long height = sl.height;
            long width = sl.width;

            double slice_area = (double)height * (double)width;
            if (accumulated + slice_area >= half) {
                return (double)start_y + (half - accumulated) / (double)width;
            }
            accumulated += slice_area;
        }

        return (double)prev_y;
    }
}