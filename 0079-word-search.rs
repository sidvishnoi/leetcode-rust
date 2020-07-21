struct Solution;

type Grid<T> = Vec<Vec<T>>;
impl Solution {
    pub fn exist(board: Grid<char>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        let (rows, cols) = (board.len(), board[0].len());

        if word.len() > rows * cols {
            return false;
        }

        let mut board = board;
        for r in 0..rows {
            for c in 0..cols {
                if Self::dfs(&mut board, r, c, 0, &word) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &mut Grid<char>, r: usize, c: usize, index: usize, word: &[char]) -> bool {
        if index == word.len() {
            return true;
        }
        if r == board.len() || c == board[0].len() || board[r][c] != word[index] {
            return false;
        }

        let old = board[r][c];
        board[r][c] = '#'; // mark as visited

        if (r > 0 && Self::dfs(board, r - 1, c, index + 1, word))
            || (c > 0 && Self::dfs(board, r, c - 1, index + 1, word))
            || Self::dfs(board, r + 1, c, index + 1, word)
            || Self::dfs(board, r, c + 1, index + 1, word)
        {
            return true;
        }

        board[r][c] = old; // unmark (backtrack)
        false
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(true, Solution::exist(board.clone(), "ABCCED".to_string()));
    assert_eq!(true, Solution::exist(board.clone(), "SEE".to_string()));
    assert_eq!(false, Solution::exist(board.clone(), "ABCB".to_string()));

    assert_eq!(true, Solution::exist(vec![vec!['a']], "a".to_string()))
}
