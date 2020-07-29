struct Solution;

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sell = 0;
        let mut hold = std::i32::MIN;
        let mut rest = 0;

        for i in 0..prices.len() {
            let prev_sell = sell;
            sell = hold + prices[i];
            hold = max(hold, rest - prices[i]);
            rest = max(rest, prev_sell);
        }
        max(sell, rest)
    }
}

fn main() {
    assert_eq!(3, Solution::max_profit(vec![1, 2, 3, 0, 2]));
}
