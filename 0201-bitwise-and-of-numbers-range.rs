struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        // find the common prefix of `m` and `n`,
        // then left-shift the common prefix `i` times to pad from right
        // example:
        // 5: 101
        // 6: 110
        // 7: 111
        //
        // 101 -> 10, 111 -> 11 (i = 1)
        //  10 ->  1,  11 ->  1 (i = 2)
        // m == n == 1, return 1 << i == 100
        let mut m = m as u32;
        let mut n = n as u32;
        let mut i = 0;
        while m != n {
            m >>= 1;
            n >>= 1;
            i += 1;
        }
        let m = m as i32;
        m << i
    }
}

fn main() {
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
    assert_eq!(0, Solution::range_bitwise_and(0, 1));
}
