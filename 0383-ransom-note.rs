struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut counts = [0i32; 26];
        for code in magazine.bytes().map(Self::as_code) {
            counts[code] += 1;
        }

        for code in ransom_note.bytes().map(Self::as_code) {
            counts[code] -= 1;
            if counts[code] < 0 {
                return false;
            }
        }
        true
    }

    #[inline]
    fn as_code(b: u8) -> usize {
        (b - 'a' as u8) as usize
    }
}

fn main() {
    assert_eq!(
        false,
        Solution::can_construct("a".to_string(), "b".to_string())
    );
    assert_eq!(
        false,
        Solution::can_construct("aa".to_string(), "ab".to_string())
    );
    assert_eq!(
        true,
        Solution::can_construct("aa".to_string(), "aab".to_string())
    );
}
