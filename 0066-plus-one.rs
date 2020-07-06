struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }

        let mut result = vec![0; digits.len() + 1];
        result[0] = 1;
        result
    }
}

fn main() {
    assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
}
