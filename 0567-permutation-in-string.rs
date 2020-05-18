struct Solution;

fn idx(ch: u8) -> usize {
    (ch - b'a') as usize
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();

        let mut s1_count = [0; 26];
        let mut s2_count = [0; 26];
        for i in 0..s1.len() {
            s1_count[idx(s1[i])] += 1;
            s2_count[idx(s2[i])] += 1;
        }

        let mut same_count = 0;
        for i in 0..26usize {
            if s1_count[i] == s2_count[i] {
                same_count += 1;
            }
        }
        for i in 0..(s2.len() - s1.len()) {
            if same_count == 26 {
                return true;
            }
            let (l, r) = (idx(s2[i]), idx(s2[i + s1.len()]));
            s2_count[r] += 1;
            if s2_count[r] == s1_count[r] {
                same_count += 1;
            } else if s2_count[r] == s1_count[r] + 1 {
                same_count -= 1;
            }
            s2_count[l] -= 1;
            if s2_count[l] == s1_count[l] {
                same_count += 1;
            } else if s2_count[l] == s1_count[l] - 1 {
                same_count -= 1;
            }
        }
        same_count == 26
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string())
    );
    assert_eq!(
        false,
        Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string())
    );
}
