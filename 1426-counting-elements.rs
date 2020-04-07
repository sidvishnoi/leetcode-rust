struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut count = 0;
        for (n, c) in &map {
            if map.contains_key(&(n + 1)) {
                count += c;
            }
        }
        count
    }
}

fn main() {
    assert_eq!(2, Solution::count_elements(vec![1, 2, 3]));
    assert_eq!(0, Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]));
    assert_eq!(3, Solution::count_elements(vec![1, 3, 2, 3, 5, 0]));
    assert_eq!(2, Solution::count_elements(vec![1, 1, 2, 2]));
}
