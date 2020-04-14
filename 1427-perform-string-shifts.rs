struct Solution;

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut net_shift = 0;
        for s in shift {
            let (dir, amount) = (s[0], s[1]);
            if dir == 0 {
                net_shift -= amount;
            } else {
                net_shift += amount;
            }
        }

        if net_shift == 0 {
            return s;
        }

        let idx = if net_shift > 0 {
            s.len() - (net_shift as usize) % s.len()
        } else {
            (net_shift.abs() as usize) % s.len()
        };
        let (front, back) = unsafe {
            let s = s.as_bytes();
            (
                String::from_utf8_unchecked(s[idx..].to_vec()),
                String::from_utf8_unchecked(s[0..idx].to_vec()),
            )
        };
        format!("{}{}", front, back)
    }
}

fn main() {
    assert_eq!(
        "cab",
        Solution::string_shift("abc".to_string(), vec![vec![0, 1], vec![1, 2]])
    );
    assert_eq!(
        "efgabcd",
        Solution::string_shift(
            "abcdefg".to_string(),
            vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
        )
    );
}
