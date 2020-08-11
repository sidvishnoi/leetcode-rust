struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut buckets = vec![0; n + 1];
        for c in citations {
            if c >= n as i32 {
                buckets[n] += 1;
            } else {
                buckets[c as usize] += 1;
            }
        }

        let mut total = 0;
        for (i, count) in buckets.into_iter().enumerate().rev() {
            total += count;
            if total >= i as i32 {
                return i as i32;
            }
        }
        0
    }
}

fn main() {
    assert_eq!(3, Solution::h_index(vec![0, 3, 6, 1, 5]));
    assert_eq!(3, Solution::h_index(vec![3, 3, 25, 8, 5]));
    assert_eq!(2, Solution::h_index(vec![0, 5, 2, 6, 1]));
    assert_eq!(1, Solution::h_index(vec![0, 1]));
    assert_eq!(0, Solution::h_index(vec![0, 0]));
    assert_eq!(1, Solution::h_index(vec![1, 1]));
    assert_eq!(1, Solution::h_index(vec![1, 1, 1]));
    assert_eq!(1, Solution::h_index(vec![100]));
    assert_eq!(0, Solution::h_index(vec![]));
}
