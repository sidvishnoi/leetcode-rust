struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        let mut root = num;
        while root * root > num {
            root = (root + num / root) / 2;
        }
        root * root == num
    }
}

fn main() {
    assert_eq!(true, Solution::is_perfect_square(16));
    assert_eq!(false, Solution::is_perfect_square(14));
}
