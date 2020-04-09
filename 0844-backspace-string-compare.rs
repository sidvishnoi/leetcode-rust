struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();

        let mut skip_s = 0;
        let mut skip_t = 0;
        let mut i = s.len() as i32 - 1;
        let mut j = t.len() as i32 - 1;

        while i >= 0 || j >= 0 {
            while i >= 0 {
                if s[i as usize] == b'#' {
                    skip_s += 1;
                    i -= 1;
                } else if skip_s > 0 {
                    skip_s -= 1;
                    i -= 1;
                } else {
                    break;
                }
            }
            while j >= 0 {
                if t[j as usize] == b'#' {
                    skip_t += 1;
                    j -= 1;
                } else if skip_t > 0 {
                    skip_t -= 1;
                    j -= 1;
                } else {
                    break;
                }
            }
            if i >= 0 && j >= 0 && s[i as usize] != t[j as usize] {
                return false;
            }
            if (i >= 0) != (j >= 0) {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        true
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string())
    );
    assert_eq!(
        true,
        Solution::backspace_compare("ab##".to_string(), "c#d#".to_string())
    );
    assert_eq!(
        true,
        Solution::backspace_compare("a##c".to_string(), "#a#c".to_string())
    );
    assert_eq!(
        false,
        Solution::backspace_compare("a#c".to_string(), "b".to_string())
    );
}
