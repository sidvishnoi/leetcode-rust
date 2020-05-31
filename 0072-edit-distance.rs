struct Solution;

use std::cmp::min;
impl Solution {
    pub fn min_distance(a: String, b: String) -> i32 {
        let a = a.into_bytes();
        let b = b.into_bytes();

        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 1..=a.len() {
            dp[i][0] = i as i32;
        }
        for j in 1..=b.len() {
            dp[0][j] = j as i32;
        }

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    let (insert, delete, replace) = (dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]);
                    dp[i][j] = 1 + min(replace, min(insert, delete));
                }
            }
        }
        dp[a.len()][b.len()]
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::min_distance("horse".to_string(), "ros".to_string())
    );
    assert_eq!(
        5,
        Solution::min_distance("intention".to_string(), "execution".to_string())
    );
}
