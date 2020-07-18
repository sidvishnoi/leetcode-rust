struct Solution;

use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(num_courses);
        let mut indegree = vec![0; num_courses];
        for prerequisite in prerequisites {
            let (dest, src) = (prerequisite[0], prerequisite[1]);
            graph.entry(src).or_default().push(dest);
            indegree[dest as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for i in 0..num_courses {
            if indegree[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        let mut topological_order = Vec::with_capacity(num_courses);
        while let Some(node) = queue.pop_front() {
            topological_order.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors {
                    let neighbor = *neighbor;
                    indegree[neighbor as usize] -= 1;
                    if indegree[neighbor as usize] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if topological_order.len() == num_courses {
            topological_order
        } else {
            Vec::new()
        }
    }
}

fn main() {
    assert_eq!(vec![0, 1], Solution::find_order(2, vec![vec![1, 0]]));
    assert_eq!(
        vec![0, 1, 2, 3],
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]])
    );
}
