struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a = text1.into_bytes();
        let b = text2.into_bytes();

        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = 1 + dp[i][j];
                } else {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }
        dp[a.len()][b.len()]
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
    assert_eq!(
        3,
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
    );
    assert_eq!(
        0,
        Solution::longest_common_subsequence("abc".to_string(), "def".to_string())
    );
}
