struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        // (we ignore all leading zeroes)
        // num  = 0000 0101 (5)
        // mask = 0000 0001 -> 0000 0011 -> 0000 0111
        // result 0000 0111 ^ 0000 0101 = 00000 0010 (2)
        let mut mask = 1;
        while mask < num {
            mask = (mask << 1) + 1;
        }
        mask ^ num
    }
}

fn main() {
    assert_eq!(2, Solution::find_complement(5));
    assert_eq!(0, Solution::find_complement(1));
}
