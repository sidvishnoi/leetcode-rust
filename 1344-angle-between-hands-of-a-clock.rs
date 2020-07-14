struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        // hour hand moves 30deg per hour (360deg / 12hour)
        // hour hand also moves 0.5deg per minute (30deg / 60min)
        let hour_hand_deg = (30 * hour) as f64 + (minutes as f64 * 0.5);
        // minute hand moves 6deg per minute (360deg / 60min)
        let min_hand_deg = (minutes * 6) as f64;

        let angle = (hour_hand_deg - min_hand_deg).abs();
        if angle > 180.0 {
            360.0 - angle
        } else {
            angle
        }
    }
}

fn main() {
    assert_eq!(165.0, Solution::angle_clock(12, 30));
    assert_eq!(75.0, Solution::angle_clock(3, 30));
    assert_eq!(7.5, Solution::angle_clock(3, 15));
    assert_eq!(155.0, Solution::angle_clock(4, 50));
    assert_eq!(0.0, Solution::angle_clock(12, 0));
}
