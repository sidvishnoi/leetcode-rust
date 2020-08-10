struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut number = 0;
        for ch in s.chars() {
            number = (number * 26) + (ch as u8 - b'A' + 1) as i32;
        }
        number
    }
}

fn main() {
    assert_eq!(1, Solution::title_to_number("A".to_string()));
    assert_eq!(28, Solution::title_to_number("AB".to_string()));
    assert_eq!(701, Solution::title_to_number("ZY".to_string()));
    assert_eq!(2147483647, Solution::title_to_number("FXSHRXW".to_string()));
}
