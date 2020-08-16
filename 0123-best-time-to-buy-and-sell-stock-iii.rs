struct Solution;

use std::cmp::{max, min};
// Time: O(n)
// Space: O(1)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_cost_one_buy = std::i32::MAX;
        let mut max_profit_one_sell = 0;
        let mut min_cost_two_buy = std::i32::MAX;
        let mut max_profit_two_sell = 0;
        for price in prices {
            min_cost_one_buy = min(min_cost_one_buy, price);
            max_profit_one_sell = max(max_profit_one_sell, price - min_cost_one_buy);
            min_cost_two_buy = min(min_cost_two_buy, price - max_profit_one_sell);
            max_profit_two_sell = max(max_profit_two_sell, price - min_cost_two_buy);
        }
        max_profit_two_sell
    }
}

fn main() {
    assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
