struct Solution;

use std::collections::{HashMap, BTreeMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut by_count = BTreeMap::new();
        for (num, count) in counts {
            by_count.entry(count).or_insert(Vec::new()).push(num);
        }

        let mut result = Vec::with_capacity(k);
        for (_count, nums) in by_count.into_iter().rev() {
            for num in nums {
                if result.len() == k {
                    break;
                }
                result.push(num);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(vec![1,2], Solution::top_k_frequent(vec![1,1,1,2,2,3], 2));
    assert_eq!(vec![1], Solution::top_k_frequent(vec![1], 1));
}
