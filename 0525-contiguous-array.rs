struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max_length = 0;
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                max_length = (i + 1) as i32;
            }
            if map.contains_key(&count) {
                let len = i - map.get(&count).unwrap();
                max_length = std::cmp::max(max_length, len as i32);
            } else {
                map.insert(count, i);
            }
        }
        max_length
    }
}

fn main() {
    assert_eq!(2, Solution::find_max_length(vec![0, 1]));
    assert_eq!(2, Solution::find_max_length(vec![0, 1, 0]));
}
