struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut counts = std::collections::HashMap::new();
        let len = s.len();

        let mut buckets = vec![Vec::new(); len + 1];
        for ch in s.into_bytes() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        for (ch, count) in counts {
            buckets[count].push(ch);
        }
        let mut result = Vec::with_capacity(len);
        for (count, bucket) in buckets.into_iter().enumerate().rev() {
            for ch in bucket {
                for _ in 0..count {
                    result.push(ch);
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

fn main() {
    assert_eq!("eert", Solution::frequency_sort("tree".to_string()));
    assert_eq!("cccaaa", Solution::frequency_sort("cccaaa".to_string()));
    assert_eq!("bbAa", Solution::frequency_sort("Aabb".to_string()));
}
