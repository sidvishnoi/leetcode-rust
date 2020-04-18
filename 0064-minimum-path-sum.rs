struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (R, C) = (grid.len(), grid[0].len());
        for r in 0..R {
            for c in 0..C {
                grid[r][c] = match (r, c) {
                    (0, 0) => grid[r][c],
                    (0, _c) => grid[r][c] + grid[r][c - 1],
                    (_r, 0) => grid[r][c] + grid[r - 1][c],
                    _ => grid[r][c] + std::cmp::min(grid[r][c - 1], grid[r - 1][c]),
                }
            }
        }
        grid[R - 1][C - 1]
    }
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(7, Solution::min_path_sum(grid));
}
