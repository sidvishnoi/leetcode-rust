struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut all_paths = Vec::new();
        let mut current_path = vec![0];
        Self::dfs(&graph, 0, &mut current_path, &mut all_paths);
        all_paths
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        current_node: usize,
        current_path: &mut Vec<i32>,
        all_paths: &mut Vec<Vec<i32>>,
    ) {
        if current_node == graph.len() - 1 {
            all_paths.push(current_path.clone());
            return;
        }

        for &next_node in &graph[current_node] {
            current_path.push(next_node);
            Self::dfs(graph, next_node as usize, current_path, all_paths);
            current_path.pop();
        }
    }
}

fn main() {
    assert_eq!(
        vec![vec![0, 1, 3], vec![0, 2, 3]],
        Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]])
    );
}
