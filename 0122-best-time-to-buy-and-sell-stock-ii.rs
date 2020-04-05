struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit_value = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max_profit_value += prices[i] - prices[i - 1];
            }
        }
        max_profit_value
    }
}

fn main() {
    assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
}
