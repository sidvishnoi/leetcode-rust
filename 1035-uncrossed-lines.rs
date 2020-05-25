struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[a.len()][b.len()]
    }
}

fn main() {
    let a = vec![1, 4, 2];
    let b = vec![1, 2, 4];
    assert_eq!(2, Solution::max_uncrossed_lines(a, b));

    let a = vec![2, 5, 1, 2, 5];
    let b = vec![10, 5, 2, 1, 5, 2];
    assert_eq!(3, Solution::max_uncrossed_lines(a, b));

    let a = vec![1, 3, 7, 1, 7, 5];
    let b = vec![1, 9, 2, 5, 1];
    assert_eq!(2, Solution::max_uncrossed_lines(a, b));
}
