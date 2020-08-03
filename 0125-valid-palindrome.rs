struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let s = s.as_bytes();
        let mut left = 0_usize;
        let mut right = s.len() - 1;
        while left < right {
            if !s[left].is_ascii_alphanumeric() {
                left += 1;
            } else if !s[right].is_ascii_alphanumeric() {
                right -= 1;
            } else if !s[left].eq_ignore_ascii_case(&s[right]) {
                return false;
            } else {
                left += 1;
                right -= 1;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
}
