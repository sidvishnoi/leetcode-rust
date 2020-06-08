struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

fn main() {
    assert_eq!(true, Solution::is_power_of_two(1));
    assert_eq!(true, Solution::is_power_of_two(16));
    assert_eq!(false, Solution::is_power_of_two(218));
}
