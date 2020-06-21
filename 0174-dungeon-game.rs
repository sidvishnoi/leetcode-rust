struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        // hp[i][j] is min hp needed when entering (i, j)
        let mut hp = vec![vec![std::i32::MAX; n + 1]; m + 1];

        // We need to get out of princess' cell with health=1. The princess's
        // cell could also have demons - which can kill the knight in that cell,
        // without being able to save princess.
        hp[m][n - 1] = 1;
        hp[m - 1][n] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let hp_needed = min(hp[i + 1][j], hp[i][j + 1]) - dungeon[i][j];
                hp[i][j] = max(1, hp_needed);
            }
        }
        hp[0][0]
    }
}

fn main() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    assert_eq!(7, Solution::calculate_minimum_hp(dungeon));
}
