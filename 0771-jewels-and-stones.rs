struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut counts = std::collections::HashMap::new();
        let j = j.into_bytes();
        let s = s.into_bytes();

        for ch in j.into_iter() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        let mut count = 0;
        for ch in s.into_iter() {
            count += *counts.get(&ch).unwrap_or(&0);
        }
        count
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string())
    );
    assert_eq!(
        0,
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string())
    );
}
