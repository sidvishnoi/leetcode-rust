struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![vec![0; amount + 1]; coins.len() + 1];
        dp[0][0] = 1;
        for c in 1..=coins.len() {
            dp[c][0] = 1; // number of ways of selecting for amount zero is 1
            for amt in 1..=amount {
                dp[c][amt] = dp[c - 1][amt];
                if (amt as i32) - coins[c - 1] >= 0 {
                    dp[c][amt] += dp[c][amt - (coins[c - 1] as usize)];
                }
            }
        }
        dp[coins.len()][amount]
    }
}

fn main() {
    assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    assert_eq!(0, Solution::change(3, vec![2]));
    assert_eq!(1, Solution::change(10, vec![10]));
}
