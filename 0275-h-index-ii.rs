struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut low = 0 as usize;
        let mut high = len;
        while low < high {
            let mid = low + (high - low) / 2;
            if citations[mid] >= (len - mid) as i32 {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        (len - low) as i32
    }
}

// https://leetcode.com/problems/h-index-ii/discuss/71063/Standard-binary-search/73201
struct Solution2;
impl Solution2 {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.reverse();
        let mut result = 0;
        for i in 0..citations.len() {
            if citations[i] < (i + 1) as i32 {
                break;
            }
            result = (i + 1) as i32;
        }
        result
    }
}

fn main() {
    assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
    assert_eq!(3, Solution::h_index(vec![3, 3, 5, 8, 25]));
    assert_eq!(2, Solution::h_index(vec![0, 1, 2, 5, 6]));
    assert_eq!(1, Solution::h_index(vec![0, 1]));
    assert_eq!(0, Solution::h_index(vec![0, 0]));
    assert_eq!(1, Solution::h_index(vec![1, 1]));
    assert_eq!(1, Solution::h_index(vec![1, 1, 1]));
    assert_eq!(1, Solution::h_index(vec![100]));
    assert_eq!(0, Solution::h_index(vec![]));
}
