#[derive(Default)]
struct Trie {
    is_word: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for ch in Self::char_indexes(word) {
            let children = &mut curr.children[ch];
            curr = children.get_or_insert_with(|| Box::new(Trie::new()))
        }
        curr.is_word = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        match self.find(word) {
            Some(t) => t.is_word,
            None => false,
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: String) -> Option<&Self> {
        let mut curr = self;
        for ch in Self::char_indexes(word) {
            curr = match &curr.children[ch] {
                Some(trie) => trie,
                None => return None,
            };
        }
        Some(curr)
    }

    #[inline]
    fn char_indexes(word: String) -> impl Iterator<Item = usize> {
        word.into_bytes().into_iter().map(|ch| (ch - b'a') as usize)
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(true, trie.search("apple".to_string()));
    assert_eq!(false, trie.search("app".to_string()));
    assert_eq!(true, trie.starts_with("app".to_string()));
    trie.insert("app".to_string());
    assert_eq!(true, trie.search("app".to_string()));
}
