struct Solution;

use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let intervals_by_start: BTreeMap<i32, usize> = intervals
            .iter()
            .enumerate()
            .map(|(i, interval)| (interval[0], i))
            .collect();

        intervals
            .into_iter()
            .map(|interval| {
                // Find the first start point >= each end point
                intervals_by_start
                    .range((Included(interval[1]), Unbounded))
                    .next()
                    .map_or(-1, |(&_start, &i)| i as i32)
            })
            .collect()
    }
}

fn main() {
    let intervals = vec![vec![1, 2]];
    assert_eq!(vec![-1], Solution::find_right_interval(intervals));

    let intervals = vec![vec![3, 4], vec![2, 3], vec![1, 2]];
    assert_eq!(vec![-1, 0, 1], Solution::find_right_interval(intervals));

    let intervals = vec![vec![1, 4], vec![2, 3], vec![3, 4]];
    assert_eq!(vec![-1, 2, -1], Solution::find_right_interval(intervals));
}
