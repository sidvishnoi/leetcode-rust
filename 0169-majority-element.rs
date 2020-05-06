struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = std::i32::MAX;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
                count += 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

fn main() {
    assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
