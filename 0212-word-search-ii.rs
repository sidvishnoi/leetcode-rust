struct Solution;

fn ord(ch: char) -> usize {
    ch as usize - 'a' as usize
}

#[derive(Default)]
struct Trie {
    word: Option<String>,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn from_words(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            let mut node = &mut trie;
            for ch in word.chars() {
                node = node.children[ord(ch)].get_or_insert_with(|| Box::new(Trie::new()));
            }
            node.word = Some(word);
        }
        trie
    }
}

type Grid<T> = Vec<Vec<T>>;
impl Solution {
    pub fn find_words(board: Grid<char>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::from_words(words);
        let mut board = board;
        let mut result: Vec<String> = Vec::new();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                Self::dfs(&mut board, i, j, &mut trie, &mut result);
            }
        }
        result
    }

    fn dfs(board: &mut Grid<char>, i: usize, j: usize, trie: &mut Trie, result: &mut Vec<String>) {
        let ch = board[i][j];
        if ch == '#' {
            return;
        }
        if let Some(trie) = trie.children[ord(ch)].as_mut() {
            board[i][j] = '#';
            if trie.word.is_some() {
                result.push(trie.word.take().unwrap());
            }
            if i > 0 {
                Self::dfs(board, i - 1, j, trie, result);
            }
            if j > 0 {
                Self::dfs(board, i, j - 1, trie, result);
            }
            if i < board.len() - 1 {
                Self::dfs(board, i + 1, j, trie, result);
            }
            if j < board[0].len() - 1 {
                Self::dfs(board, i, j + 1, trie, result);
            }
            board[i][j] = ch;
        }
    }
}

fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    assert_eq!(vec!["oath", "eat"], Solution::find_words(board, words));
}
