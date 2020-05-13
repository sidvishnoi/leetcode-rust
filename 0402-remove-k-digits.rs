struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let num = num.into_bytes();
        let mut k = k;

        let count = num.len() - k as usize;

        let mut stack = Vec::with_capacity(num.len());
        for ch in num {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > ch {
                stack.pop();
                k -= 1;
            }
            stack.push(ch);
        }

        let chars: Vec<_> = stack
            .into_iter()
            .skip_while(|ch| *ch == b'0')
            .take(count)
            .collect();
        if chars.is_empty() {
            return String::from("0");
        }
        String::from_utf8(chars).unwrap()
    }
}

fn main() {
    assert_eq!("1219", Solution::remove_kdigits(String::from("1432219"), 3));
    assert_eq!("200", Solution::remove_kdigits(String::from("10200"), 1));
    assert_eq!("0", Solution::remove_kdigits(String::from("10"), 2));
    assert_eq!("111", Solution::remove_kdigits(String::from("11111"), 2));
    assert_eq!("11111", Solution::remove_kdigits(String::from("11111"), 0));
    assert_eq!("0", Solution::remove_kdigits(String::from("11111"), 5));
    assert_eq!("0", Solution::remove_kdigits(String::from("12345"), 5));
    assert_eq!("12345", Solution::remove_kdigits(String::from("12345"), 0));
    assert_eq!("12", Solution::remove_kdigits(String::from("12345"), 3));
    assert_eq!("0", Solution::remove_kdigits(String::from("1234567890"), 9));
}
