#[derive(Default)]
struct TrieNode {
    is_word: bool,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Default)]
struct WordDictionary {
    root: Box<TrieNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        for ch in word.into_bytes().into_iter().map(|ch| (ch - b'a') as usize) {
            let children = &mut curr.children[ch];
            curr = children.get_or_insert_with(|| Box::new(TrieNode::new()))
        }
        curr.is_word = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        Self::_search(&word.as_bytes(), 0, &self.root)
    }

    fn _search(word: &[u8], i: usize, root: &Box<TrieNode>) -> bool {
        if i == word.len() {
            return root.is_word;
        }

        if word[i] != b'.' {
            let idx = (word[i] - b'a') as usize;
            return if let Some(Some(curr)) = root.children.get(idx) {
                Self::_search(word, i + 1, curr)
            } else {
                false
            };
        } else {
            // Encountered a `.`, perform a DFS in all children.
            for idx in 0..26_usize {
                if let Some(Some(curr)) = root.children.get(idx) {
                    if Self::_search(word, i + 1, curr) {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn main() {
    let mut word_dict = WordDictionary::new();
    word_dict.add_word("bad".to_string());
    word_dict.add_word("dad".to_string());
    word_dict.add_word("mad".to_string());
    assert_eq!(false, word_dict.search("pad".to_string()));
    assert_eq!(true, word_dict.search("bad".to_string()));
    assert_eq!(true, word_dict.search(".ad".to_string()));
    assert_eq!(true, word_dict.search("b..".to_string()));
    assert_eq!(true, word_dict.search("...".to_string()));
    assert_eq!(false, word_dict.search("b...".to_string()));
    assert_eq!(false, word_dict.search(".".to_string()));
}
