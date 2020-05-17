struct Solution;

fn idx(ch: u8) -> usize {
    (ch - b'a') as usize
}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();
        if p.len() > s.len() {
            return result;
        }

        let s = s.into_bytes();
        let p = p.into_bytes();

        let mut p_counts = [0; 26];
        for ch in &p[..] {
            p_counts[idx(*ch)] += 1;
        }

        let mut window = [0; 26];
        for ch in &s[0..(p.len() - 1)] {
            window[idx(*ch)] += 1;
        }

        let mut left: usize = 0;
        let mut right = p.len() - 1;
        let k = s.len() - p.len() + 1;
        while left < k {
            window[idx(s[right])] += 1;
            // is anagram?
            if window == p_counts {
                result.push(left as i32);
            }
            window[idx(s[left])] -= 1;
            left += 1;
            right += 1;
        }

        result
    }
}

fn main() {
    assert_eq!(
        vec![0, 6],
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
    );
    assert_eq!(
        vec![0, 1, 2],
        Solution::find_anagrams("abab".to_string(), "ab".to_string())
    );
}
