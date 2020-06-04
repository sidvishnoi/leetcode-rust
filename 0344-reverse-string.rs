struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let mut i: usize = 0;
        let mut j = s.len() - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s);

    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut s);
    assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s);
}
