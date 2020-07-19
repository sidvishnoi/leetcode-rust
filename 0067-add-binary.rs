struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();

        let mut i = a.len();
        let mut j = b.len();

        let mut result: Vec<u8> = Vec::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
        let mut carry = 0;
        while i > 0 || j > 0 || carry != 0 {
            if i > 0 {
                i -= 1;
                carry += a[i] - b'0';
            }
            if j > 0 {
                j -= 1;
                carry += b[j] - b'0';
            }
            result.push(carry % 2 + b'0');
            carry /= 2;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    assert_eq!(
        "100",
        Solution::add_binary("11".to_string(), "1".to_string())
    );
    assert_eq!(
        "10101",
        Solution::add_binary("1010".to_string(), "1011".to_string())
    );
}
