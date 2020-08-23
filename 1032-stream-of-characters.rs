use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Default)]
struct TrieNode {
    is_word: bool,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: Vec<u8>) {
        let mut curr = self;
        for ch in word.into_iter().map(|ch| (ch - b'a') as usize) {
            let children = &mut curr.children[ch];
            curr = children.get_or_insert_with(|| Box::new(TrieNode::new()))
        }
        curr.is_word = true;
    }
}

struct StreamChecker {
    trie: TrieNode,
    last_queried: RefCell<VecDeque<u8>>,
    max_word_length: usize,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let max_word_length = words.iter().map(|word| word.len()).max().unwrap_or(0_usize);
        let last_queried = VecDeque::with_capacity(max_word_length);

        let mut trie = TrieNode::new();
        for word in words {
            let mut word = word.into_bytes();
            word.reverse();
            trie.insert(word);
        }

        Self {
            trie,
            last_queried: RefCell::new(last_queried),
            max_word_length,
        }
    }

    fn query(&self, letter: char) -> bool {
        self.last_queried.borrow_mut().push_back(letter as u8);
        if self.last_queried.borrow().len() > self.max_word_length {
            self.last_queried.borrow_mut().pop_front();
        }

        let mut curr = &self.trie;
        for &ch in self.last_queried.borrow().iter().rev() {
            curr = match &curr.children[(ch - b'a') as usize] {
                Some(trie) if trie.is_word => return true,
                Some(trie) => trie,
                None => break,
            };
        }
        false
    }
}

fn main() {
    let sc = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(false, sc.query('a'));
    assert_eq!(false, sc.query('b'));
    assert_eq!(false, sc.query('c'));
    assert_eq!(true, sc.query('d'));
    assert_eq!(false, sc.query('e'));
    assert_eq!(true, sc.query('f'));
    assert_eq!(false, sc.query('g'));
    assert_eq!(false, sc.query('h'));
    assert_eq!(false, sc.query('i'));
    assert_eq!(false, sc.query('j'));
    assert_eq!(false, sc.query('k'));
    assert_eq!(true, sc.query('l'));

    let sc = StreamChecker::new(vec!["abcde".to_string(), "ef".to_string()]);
    assert_eq!(false, sc.query('a'));
    assert_eq!(false, sc.query('b'));
    assert_eq!(false, sc.query('c'));
    assert_eq!(false, sc.query('d'));
    assert_eq!(true, sc.query('e'));
    assert_eq!(true, sc.query('f'));
    assert_eq!(false, sc.query('g'));
}
