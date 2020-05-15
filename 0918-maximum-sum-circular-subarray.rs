struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        // We want to find:
        // max(the max subarray sum, the total sum - the min subarray sum)
        use std::cmp::{max, min};

        let mut total = 0;
        let mut max_sum = std::i32::MIN;
        let mut current_max = 0;
        let mut min_sum = std::i32::MAX;
        let mut current_min = 0;

        for num in a {
            current_max = max(current_max + num, num);
            max_sum = max(max_sum, current_max);

            current_min = min(current_min + num, num);
            min_sum = min(min_sum, current_min);

            total += num;
        }

        if min_sum == total {
            // all numbers are negative, then max_sum is the largest number
            max_sum
        } else {
            max(max_sum, total - min_sum)
        }
    }
}

fn main() {
    assert_eq!(3, Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]));
    assert_eq!(10, Solution::max_subarray_sum_circular(vec![5, -3, 5]));
    assert_eq!(4, Solution::max_subarray_sum_circular(vec![3, -1, 2, -1]));
    assert_eq!(3, Solution::max_subarray_sum_circular(vec![3, -2, 2, -3]));
    assert_eq!(-1, Solution::max_subarray_sum_circular(vec![-2, -3, -1]));
}
