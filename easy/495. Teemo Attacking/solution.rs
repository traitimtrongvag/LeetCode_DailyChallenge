// Algorithm overview:
// Iterate through attack timestamps.
// For each attack, add the effective poisoned time until the next attack.
// Overlaps are handled by taking the smaller value between gap and duration.
// Finally, add the duration of the last attack.

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        // Accumulates the total poisoned time.
        let mut total = 0;

        // Process all attacks except the last one.
        for i in 0..(time_series.len() - 1) {
            // Add the non-overlapping poisoned time.
            total += std::cmp::min(time_series[i + 1] - time_series[i], duration);
        }

        // Add the full duration of the last attack.
        total + duration
    }
}