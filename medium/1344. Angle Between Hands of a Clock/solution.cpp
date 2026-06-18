class Solution {
public:
    double angleClock(int hour, int minutes) {
        // Minute hand: 360° / 60 = 6° per minute
        double minuteAngle = minutes * 6.0;
        
        // Hour hand: 360° / 12 = 30° per hour + 0.5° per minute
        double hourAngle = (hour % 12) * 30.0 + minutes * 0.5;
        
        double diff = abs(hourAngle - minuteAngle);
        
        return min(diff, 360.0 - diff);
    }
};