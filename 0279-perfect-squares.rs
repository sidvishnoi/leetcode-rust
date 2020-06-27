struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        // dp[i] is least number of perfect squares that sum to i.
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let sqrt = (i as f64).sqrt() as usize;

            // optimisation: if number is already a perfect square, then dp[i] is 1.
            if sqrt * sqrt == i {
                dp[i] = 1;
                continue;
            }

            // For each i, it must be the sum of some number (i - j*j) and a perfect square number (j*j).
            // 1 + min(dp[n - 1], dp[n - 4], dp[n - 9], dp[n - 16], ...)
            for j in 1..=sqrt {
                dp[i] = std::cmp::min(dp[i], 1 + dp[i - j * j]);
            }
        }
        dp[n]
    }
}

fn main() {
    assert_eq!(3, Solution::num_squares(12));
    assert_eq!(2, Solution::num_squares(13));
}
