struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .fold(0, |num, ch| (num * 26) + (ch - b'A' + 1) as i32)
    }
}

fn main() {
    assert_eq!(1, Solution::title_to_number("A".to_string()));
    assert_eq!(28, Solution::title_to_number("AB".to_string()));
    assert_eq!(701, Solution::title_to_number("ZY".to_string()));
    assert_eq!(2147483647, Solution::title_to_number("FXSHRXW".to_string()));
}
