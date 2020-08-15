struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // Sort to make first-to-end come first.
        intervals.sort_by_key(|interval| interval[1]);

        let mut finish_time = std::i32::MIN;
        let mut count = 0;
        for interval in intervals.into_iter() {
            let (start, end) = (interval[0], interval[1]);
            if start < finish_time {
                count += 1;
            } else {
                finish_time = end;
            }
        }
        count
    }
}

fn main() {
    assert_eq!(
        1,
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
    );
    assert_eq!(
        2,
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]])
    );
    assert_eq!(
        0,
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
    );
}
