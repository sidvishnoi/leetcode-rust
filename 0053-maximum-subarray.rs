struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = std::i32::MIN;
        for num in nums {
            sum += num;
            max_sum = std::cmp::max(max_sum, sum);
            if sum < 0 {
                sum = 0;
            }
        }
        max_sum
    }
}

fn main() {
    assert_eq!(
        6,
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    )
}
