struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let num_tasks = tasks.len() as i32;

        let mut frequency = vec![0; 26];
        for task in tasks {
            let task_id = task as usize - b'A' as usize;
            frequency[task_id] += 1;
        }

        frequency.sort();
        let max_frequency = *frequency.last().unwrap();
        let mut idle_slots = (max_frequency - 1) * n;
        for count in frequency.into_iter().rev().skip(1) {
            idle_slots -= std::cmp::min(max_frequency - 1, count);
        }
        if idle_slots < 0 {
            idle_slots = 0;
        }

        num_tasks + idle_slots
    }
}

fn main() {
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(8, Solution::least_interval(tasks, 2));

    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(6, Solution::least_interval(tasks, 0));

    let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
    assert_eq!(16, Solution::least_interval(tasks, 2));
}
