struct Solution;

const VOWELS: [u8; 10] = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let mut result = Vec::new();
        let mut repeat = 1_usize;
        for word in s.split_ascii_whitespace() {
            if VOWELS.iter().any(|&v| v == word.as_bytes()[0]) {
                result.push(word.to_string());
            } else {
                let (left, right) = word.split_at(1);
                result.push(format!("{}{}", right, left));
            }
            if let Some(last) = result.last_mut() {
                *last = format!("{}ma{}", last, "a".repeat(repeat));
            }
            repeat += 1;
        }
        result.join(" ")
    }
}

fn main() {
    assert_eq!(
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string(),
        Solution::to_goat_latin("I speak Goat Latin".to_string())
    );
    assert_eq!(
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string(),
        Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string())
    );
}
