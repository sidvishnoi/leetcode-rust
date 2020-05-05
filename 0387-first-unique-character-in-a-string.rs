struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let s = s.into_bytes();

        let mut counts = std::collections::HashMap::new();
        for ch in s.iter() {
            *counts.entry(*ch).or_insert(0) += 1
        }
        for i in 0..s.len() {
            if let Some(1) = counts.get(&s[i]) {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
    assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
}
