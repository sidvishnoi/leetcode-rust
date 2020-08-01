struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }
        let word = word.as_bytes();

        // Case: All capitals
        if word[0].is_ascii_uppercase() && word[1].is_ascii_uppercase() {
            for i in 2..word.len() {
                if !word[i].is_ascii_uppercase() {
                    return false;
                }
            }
        } else {
            // Case: All not capital
            // Case: All not capital except first
            for i in 1..word.len() {
                if word[i].is_ascii_uppercase() {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    assert_eq!(true, Solution::detect_capital_use("USA".to_string()));
    assert_eq!(false, Solution::detect_capital_use("FlaG".to_string()));
    assert_eq!(true, Solution::detect_capital_use("leetcode".to_string()));
    assert_eq!(true, Solution::detect_capital_use("Google".to_string()));
}
