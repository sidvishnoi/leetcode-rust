struct Solution;

use std::cmp;
// With k = max_transactions and n = number of days (length of prices)
// Time: O(kn)
// Space: O(k)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let max_transactions: usize = 2;

        // profits[k][i] = profit with k transactions until i'th day.
        // Optimization: As the profit for a day only depends on previous day,
        // we don't need to store profit for all days.
        let mut profits = vec![0; max_transactions + 1];

        // profits[k][i] = max(
        //     // if we don't trade, profit is same as previous day:
        //     profits[k, i - 1],
        //     // if we bought previous share on j'th day and sold on i'th day:
        //     prices[i] - prices[j] + profits[k-1][j-1] for j=[0..i-1]
        //     // ...which is same as: subtracting min of (prices[j] + profits[k-1][j-1]) from prices[i]
        //     // also, we can avoid repeating calculation of min above.
        // )
        let mut min = vec![prices[0]; max_transactions + 1];
        for i in 1..prices.len() {
            for k in 1..=max_transactions {
                min[k] = cmp::min(min[k], prices[i] - profits[k - 1]);
                profits[k] = cmp::max(profits[k], prices[i] - min[k]);
            }
        }

        profits[max_transactions]
    }
}

fn main() {
    assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
