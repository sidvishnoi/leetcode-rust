struct Solution;

use std::collections::HashMap;

type Memo<'a> = HashMap<usize, Vec<Vec<&'a str>>>;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut memo: Memo = HashMap::new();
        let broken_words = Self::dfs(&s, 0, &word_dict, &mut memo);
        broken_words
            .into_iter()
            .map(|mut words| {
                words.reverse();
                words.join(" ")
            })
            .collect()
    }

    fn dfs<'a>(
        s: &str,
        start: usize,
        dict: &'a Vec<String>,
        memo: &mut Memo<'a>,
    ) -> Vec<Vec<&'a str>> {
        if let Some(words) = memo.get(&start) {
            return words.clone();
        }
        if start == s.len() {
            return vec![vec![]];
        }

        let mut words: Vec<Vec<&'a str>> = Vec::new();
        let current = &s[start..];
        for word in dict {
            if current.starts_with(word) {
                for mut suffix in Self::dfs(s, start + word.len(), dict, memo) {
                    suffix.push(word);
                    words.push(suffix);
                }
            }
        }
        memo.insert(start, words.clone());
        words
    }
}

fn main() {
    let s = "catsanddog".to_string();
    let word_dict = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string(),
    ];
    assert_eq!(
        vec!["cat sand dog".to_string(), "cats and dog".to_string()],
        Solution::word_break(s, word_dict)
    );

    let s = "pineapplepenapple".to_string();
    let word_dict = vec![
        "apple".to_string(),
        "pen".to_string(),
        "applepen".to_string(),
        "pine".to_string(),
        "pineapple".to_string(),
    ];
    assert_eq!(
        vec![
            "pine apple pen apple".to_string(),
            "pine applepen apple".to_string(),
            "pineapple pen apple".to_string(),
        ],
        Solution::word_break(s, word_dict)
    );

    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    assert_eq!(Vec::<String>::new(), Solution::word_break(s, word_dict));

    let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    let word_dict = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
        "aaaaa".to_string(),
        "aaaaaa".to_string(),
        "aaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
        "aaaaaaaaa".to_string(),
        "aaaaaaaaaa".to_string(),
    ];
    assert_eq!(Vec::<String>::new(), Solution::word_break(s, word_dict));
}
