struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}

fn main() {
    assert_eq!(5, Solution::num_trees(3));
}
