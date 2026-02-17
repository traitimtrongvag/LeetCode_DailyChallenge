impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        // Vector to store all valid time strings
        let mut result = Vec::new();

        // Iterate through all possible hours (0–11)
        for hour in 0u32..12 {
            // Iterate through all possible minutes (0–59)
            for minute in 0u32..60 {

                // count_ones() returns number of 1-bits in binary representation
                // Total LEDs ON = bits in hour + bits in minute
                if (hour.count_ones() + minute.count_ones()) as i32 == turned_on {

                    // Format time:
                    // {:02} ensures minute is always 2 digits (e.g., 3 -> "03")
                    result.push(format!("{}:{:02}", hour, minute));
                }
            }
        }

        // Return all valid times
        result
    }
}