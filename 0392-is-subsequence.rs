struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s = s.into_bytes();
        let mut i: usize = 0;
        for ch in t.into_bytes() {
            if ch == s[i] {
                i += 1;
                if i == s.len() {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
    assert_eq!(
        false,
        Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string())
    );
}
