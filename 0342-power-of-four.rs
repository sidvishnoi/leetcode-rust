struct Solution;

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        // (num & (num - 1)) == 0: is power of two
        // (num & 0x55555555) != 0: 0x55 == 01010101
        num > 0 && (num & (num - 1)) == 0 && (num & 0x55555555) != 0
    }
}

fn main() {
    assert_eq!(true, Solution::is_power_of_four(16));
    assert_eq!(false, Solution::is_power_of_four(5));
    assert_eq!(false, Solution::is_power_of_four(0));
    assert_eq!(true, Solution::is_power_of_four(1));
}
