struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut a = 1;
        let mut b = 2;
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

fn main() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
}
