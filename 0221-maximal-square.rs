struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let (R, C) = (matrix.len(), matrix[0].len());

        // dp[r][c] represents the side length of maximum square
        // whose bottom-right corner is at [r][c]
        let mut dp = vec![vec![0; C + 1]; R + 1];
        let mut max_side_length = 0;

        for r in 1..=R {
            for c in 1..=C {
                if matrix[r - 1][c - 1] == '1' {
                    // If current entry is '1', see if we can extend the previous square.
                    // We can only extend the square by adding 1 (current '1' entry) to the minimum of previous sides.
                    // min(dp[r][c-1], dp[r-1][c], dp[r-1][c-1])
                    dp[r][c] = min(min(dp[r][c - 1], dp[r - 1][c]), dp[r - 1][c - 1]) + 1;
                    max_side_length = max(max_side_length, dp[r][c]);
                }
            }
        }

        max_side_length * max_side_length
    }
}

fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    assert_eq!(4, Solution::maximal_square(matrix));
}
