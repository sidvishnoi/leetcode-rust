struct Solution;

use std::cmp::max;
// Time: O(n)
// Space: O(1)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut one_buy = std::i32::MIN;
        let mut one_buy_one_sell = 0;
        let mut two_buy = std::i32::MIN;
        let mut two_buy_two_sell = 0;
        for price in prices {
            one_buy = max(one_buy, -price);
            one_buy_one_sell = max(one_buy_one_sell, price + one_buy);
            two_buy = max(two_buy, one_buy_one_sell - price);
            two_buy_two_sell = max(two_buy_two_sell, two_buy + price);
        }
        two_buy_two_sell
    }
}

fn main() {
    assert_eq!(6, Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
