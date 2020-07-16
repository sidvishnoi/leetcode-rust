struct Solution;

macro_rules! assert_almost_eq {
    ($x:expr, $y:expr) => {
        if !($x - $y < 0.00001 || $y - $x < 0.00001) {
            panic!();
        }
    };
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            return 1.0 / x * Self::my_pow(1.0 / x, -(n + 1));
        }
        if n % 2 == 0 {
            Self::my_pow(x * x, n / 2)
        } else {
            x * Self::my_pow(x * x, n / 2)
        }
    }
}

fn main() {
    assert_almost_eq!(2.0_f64.powi(10), Solution::my_pow(2.0, 10));
    assert_almost_eq!(1.1_f64.powi(3), Solution::my_pow(2.1, 3));
    assert_almost_eq!(2.0_f64.powi(-2), Solution::my_pow(2.0, -2));
}
