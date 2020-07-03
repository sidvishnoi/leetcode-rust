struct Solution;

use std::collections::HashSet;

fn hash(cells: &[i32; 8]) -> i32 {
    let mut result = 0;
    for i in 0..8_usize {
        result |= cells[i] << i;
    }
    result
}

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut cells = [
            cells[0], cells[1], cells[2], cells[3], cells[4], cells[5], cells[6], cells[7],
        ];
        let mut seen: HashSet<i32> = HashSet::new();
        let mut cycle_length = 0;
        for _ in 0..n {
            let next_day = Self::next_day(cells);
            let key = hash(&next_day);
            if !seen.insert(key) {
                break;
            }
            cycle_length += 1;
            cells = next_day;
        }

        let mut result = cells;
        if cycle_length != 0 {
            for _ in 0..(n % cycle_length) {
                result = Self::next_day(result);
            }
        }
        result.to_vec()
    }

    fn next_day(cells: [i32; 8]) -> [i32; 8] {
        let mut result = [0; 8];
        for i in 1..=6_usize {
            result[i] = (cells[i - 1] == cells[i + 1]) as i32;
        }
        result
    }
}

fn main() {
    assert_eq!(
        vec![0, 0, 1, 1, 0, 0, 0, 0],
        Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7)
    );
    assert_eq!(
        vec![0, 0, 1, 1, 1, 1, 1, 0],
        Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000)
    );
}
