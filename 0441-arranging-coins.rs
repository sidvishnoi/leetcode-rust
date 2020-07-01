struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // we're looking for a k, such that k(k+1)/2 <= n
        // or, k^2 + k - 2n = 0
        // or, (k + 1/2)^2 - 1/4 <= 2n
        // or, (k + 1/2) = sqrt(2n + 1/4)
        // or, k = sqrt(2n + 1/4) - 1/2
        let n = n as f64;
        ((2.0 * n + 1.0 / 4.0).sqrt() - 1.0 / 2.0) as i32
    }
}

fn main() {
    assert_eq!(2, Solution::arrange_coins(5));
    assert_eq!(3, Solution::arrange_coins(8));
}
