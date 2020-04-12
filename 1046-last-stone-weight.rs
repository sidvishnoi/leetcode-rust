struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones); // max heap, in-place, O(n)
        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            if a != b {
                heap.push((a - b).abs());
            }
        }
        heap.pop().unwrap_or(0)
    }
}

fn main() {
    assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
}
