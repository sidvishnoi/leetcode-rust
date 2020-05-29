struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let graph = Self::build_graph(num_courses, prerequisites);
        let mut in_degrees = Self::compute_indegrees(&graph);
        for _ in 0..num_courses {
            let mut course: usize = 0;
            while course < num_courses {
                if in_degrees[course] == 0 {
                    break;
                }
                course += 1;
            }
            if course == num_courses {
                return false;
            }
            in_degrees[course] -= 1;
            for p in &graph[course] {
                in_degrees[*p as usize] -= 1;
            }
        }

        true
    }

    fn build_graph(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; num_courses];
        for p in prerequisites {
            let (a, b) = (p[0], p[1]);
            graph[b as usize].push(a);
        }
        graph
    }

    fn compute_indegrees(graph: &Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegrees = vec![0; graph.len()];
        for adj in graph {
            for node in adj {
                indegrees[*node as usize] += 1;
            }
        }
        indegrees
    }
}

fn main() {
    assert_eq!(true, Solution::can_finish(2, vec![vec![1, 0]]));
    assert_eq!(false, Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
}
