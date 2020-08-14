struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts = [0; 64];
        for ch in s.into_bytes() {
            counts[(ch - b'A') as usize] += 1;
        }
        let mut longest_palindrome_length = 0;
        for count in counts.iter() {
            longest_palindrome_length += (count / 2) * 2;
            if longest_palindrome_length % 2 == 0 && count % 2 != 0 {
                longest_palindrome_length += 1;
            }
        }
        longest_palindrome_length
    }
}

fn main() {
    assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
    assert_eq!(9, Solution::longest_palindrome("abccccddEE".to_string()));
    assert_eq!(1, Solution::longest_palindrome("Aa".to_string()));
    assert_eq!(2, Solution::longest_palindrome("bb".to_string()));
    assert_eq!(1, Solution::longest_palindrome("zZ".to_string()));
}
