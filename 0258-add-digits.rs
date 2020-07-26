struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num > 9 {
            Self::add_digits(num / 10 + num % 10)
        } else {
            num
        }
    }
}

fn main() {
    assert_eq!(2, Solution::add_digits(38));
    assert_eq!(9, Solution::add_digits(999));
    assert_eq!(1, Solution::add_digits(1234));
}
