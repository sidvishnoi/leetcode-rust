struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }
        let (rows, cols) = (board.len(), board[0].len());

        // Step 1: On all 4 borders of board, change 'O' and all its neighbor
        // 'O' elements to a temporary '*'.
        for r in 0..rows {
            if board[r][0] == 'O' {
                Self::dfs(board, r, 0, rows, cols);
            }
            if board[r][cols - 1] == 'O' {
                Self::dfs(board, r, cols - 1, rows, cols);
            }
        }
        for c in 0..cols {
            if board[0][c] == 'O' {
                Self::dfs(board, 0, c, rows, cols);
            }
            if board[rows - 1][c] == 'O' {
                Self::dfs(board, rows - 1, c, rows, cols);
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                board[r][c] = match board[r][c] {
                    // Step 2: change all (remaining) 'O' to 'X'
                    'O' => 'X',
                    // Step 3: change all temporary '*' back to 'O'
                    '*' => 'O',
                    ch @ _ => ch,
                };
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if board[r][c] != 'O' {
            return;
        }

        board[r][c] = '*';
        if r + 1 < rows {
            Self::dfs(board, r + 1, c, rows, cols);
        }
        if c + 1 < cols {
            Self::dfs(board, r, c + 1, rows, cols);
        }
        if r > 0 {
            Self::dfs(board, r - 1, c, rows, cols);
        }
        if c > 0 {
            Self::dfs(board, r, c - 1, rows, cols);
        }
    }
}

fn convert(board: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in board.trim().split('\n') {
        let mut row = Vec::new();
        for ch in line.chars() {
            if ch.is_ascii_alphabetic() {
                row.push(ch);
            }
        }
        result.push(row);
    }
    result
}

fn main() {
    let mut board = convert(
        r#"
        X X X X
        X O O X
        X X O X
        X O X X
        "#,
    );
    let result = convert(
        r#"
        X X X X
        X X X X
        X X X X
        X O X X
        "#,
    );
    Solution::solve(&mut board);
    assert_eq!(result, board);
}
