struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .map(|word| word.split("").collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    assert_eq!(
        "blue is sky the",
        Solution::reverse_words("the sky is blue".to_string())
    );
    assert_eq!(
        "world! hello",
        Solution::reverse_words("  hello world!  ".to_string())
    );
    assert_eq!(
        "example good a",
        Solution::reverse_words("a good   example".to_string())
    );
}
