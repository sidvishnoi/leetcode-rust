struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }

        let mut grid: Vec<Vec<u8>> = grid
            .into_iter()
            .map(|row| row.into_iter().map(|c| c as u8 - b'0').collect())
            .collect();
        let (R, C) = (grid.len(), grid[0].len());

        let mut count = 0;
        for r in 0..R {
            for c in 0..C {
                if grid[r][c] == 1 {
                    Self::mark(&mut grid, r, c, R, C);
                    count += 1;
                }
            }
        }
        count
    }

    fn mark(grid: &mut Vec<Vec<u8>>, r: usize, c: usize, R: usize, C: usize) {
        if grid[r][c] != 1 {
            return;
        }
        grid[r][c] = 2;
        if r + 1 < R {
            Self::mark(grid, r + 1, c, R, C);
        }
        if r > 0 {
            Self::mark(grid, r - 1, c, R, C);
        }
        if c + 1 < C {
            Self::mark(grid, r, c + 1, R, C);
        }
        if c > 0 {
            Self::mark(grid, r, c - 1, R, C);
        }
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(1, Solution::num_islands(grid));

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(3, Solution::num_islands(grid));
}
