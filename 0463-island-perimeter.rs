struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut islands = 0;
        let mut neighbors = 0;
        let (rr, cc) = (grid.len(), grid[0].len());
        for r in 0..rr {
            for c in 0..cc {
                if grid[r][c] == 1 {
                    islands += 1;
                    if r < rr - 1 && grid[r + 1][c] == 1 {
                        neighbors += 1; // down neighbors
                    }
                    if c < cc - 1 && grid[r][c + 1] == 1 {
                        neighbors += 1; // right neighbors
                    }
                }
            }
        }
        // Each adjacent island makes 2 sides disappear:
        // +--+     +--+                   +--+--+
        // |  |  +  |  |          ->       |     |
        // +--+     +--+                   +--+--+
        islands * 4 - neighbors * 2
    }
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(16, Solution::island_perimeter(grid));
}
