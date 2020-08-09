struct Solution;

use std::collections::VecDeque;

const FRESH: i32 = 1;
const ROTTEN: i32 = 2;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut rotting = VecDeque::new();
        let mut count_fresh = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == ROTTEN {
                    rotting.push_back((r, c));
                } else if grid[r][c] == FRESH {
                    count_fresh += 1;
                }
            }
        }

        if rotting.is_empty() && count_fresh > 0 {
            return -1;
        }

        let mut minutes = 0;
        while !rotting.is_empty() && count_fresh > 0 {
            let mut just_rotted = 0;
            for _ in 0..rotting.len() {
                let (rr, cc) = rotting.pop_front().unwrap();
                for (r, c) in Self::get_adjacent_positions(rr, cc, &grid) {
                    if grid[r][c] == FRESH {
                        grid[r][c] = ROTTEN;
                        rotting.push_back((r, c));
                        just_rotted += 1;
                    }
                }
            }
            if just_rotted == 0 {
                return -1;
            }
            count_fresh -= just_rotted;
            minutes += 1;
        }

        minutes
    }

    fn get_adjacent_positions(rr: usize, cc: usize, grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for &(dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (dr < 0 && rr == 0) || (dc < 0 && cc == 0) {
                continue;
            }
            let r = (rr as i32 + dr) as usize;
            let c = (cc as i32 + dc) as usize;
            if r < grid.len() && c < grid[0].len() {
                positions.push((r, c));
            }
        }
        positions
    }
}

fn main() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    assert_eq!(4, Solution::oranges_rotting(grid));

    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    assert_eq!(-1, Solution::oranges_rotting(grid));

    let grid = vec![vec![0, 2]];
    assert_eq!(0, Solution::oranges_rotting(grid));

    let grid = vec![vec![1]];
    assert_eq!(-1, Solution::oranges_rotting(grid));

    let grid = vec![vec![0]];
    assert_eq!(0, Solution::oranges_rotting(grid));
}
