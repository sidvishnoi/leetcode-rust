struct Solution;

use std::collections::HashSet;

const SQUARES: [i32; 10] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81];

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut cycle = HashSet::new();
        while cycle.insert(n) {
            let sum = Self::square_sum(n);
            if sum == 1 {
                return true;
            }
            n = sum;
        }
        false
    }

    fn square_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut n = n;
        while n > 0 {
            sum += SQUARES[(n % 10) as usize];
            n = n / 10;
        }
        sum
    }
}

fn main() {
    assert_eq!(true, Solution::is_happy(19));
}
