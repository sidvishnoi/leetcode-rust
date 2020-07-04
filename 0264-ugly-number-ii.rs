struct Solution;

use std::cmp::min;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;

        let mut ugly_numbers = Vec::with_capacity(n);
        ugly_numbers.push(1);

        let mut p2: usize = 0;
        let mut p3: usize = 0;
        let mut p5: usize = 0;
        for _ in 1..n {
            let num = min(
                ugly_numbers[p2] * 2,
                min(ugly_numbers[p3] * 3, ugly_numbers[p5] * 5),
            );
            ugly_numbers.push(num);
            if num == ugly_numbers[p2] * 2 {
                p2 += 1;
            }
            if num == ugly_numbers[p3] * 3 {
                p3 += 1;
            }
            if num == ugly_numbers[p5] * 5 {
                p5 += 1;
            }
        }
        ugly_numbers[n - 1]
    }
}

fn main() {
    assert_eq!(12, Solution::nth_ugly_number(10));
}
