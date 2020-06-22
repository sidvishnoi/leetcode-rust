struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let mask = 1 << i;
            let count_one = nums.iter().filter(|&n| n & mask != 0).count();
            if count_one % 3 == 1 {
                result |= mask;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    assert_eq!(99, Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
}
