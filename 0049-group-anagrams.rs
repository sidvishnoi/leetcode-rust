struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for s in strs.into_iter() {
            map.entry(Self::hash(&s)).or_insert(vec![]).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }

    fn hash(s: &str) -> [u32; 26] {
        s.bytes().fold([0; 26], |mut buckets, c| {
            buckets[(c - b'a') as usize] += 1u32;
            buckets
        })
    }
}

fn main() {
    assert_eq!(
        vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]],
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ])
    );
}
