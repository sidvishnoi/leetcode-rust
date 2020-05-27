struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let graph = Self::build_graph(n, dislikes);

        let mut colors = HashMap::new();
        for node in 1..=n {
            if !colors.contains_key(&node) && !Self::dfs(node, 0, &graph, &mut colors) {
                return false;
            }
        }
        true
    }

    fn build_graph(n: usize, dislikes: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n + 1];
        for dislike in dislikes {
            let (a, b) = (dislike[0] as usize, dislike[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        }
        graph
    }

    fn dfs(
        node: usize,
        color: i32,
        graph: &Vec<Vec<usize>>,
        colors: &mut HashMap<usize, i32>,
    ) -> bool {
        if let Some(c) = colors.get(&node) {
            return *c == color;
        }
        colors.insert(node, color);
        for neighbor in &graph[node] {
            if !Self::dfs(*neighbor, color ^ 1, graph, colors) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
    assert_eq!(true, Solution::possible_bipartition(4, dislikes));

    let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(false, Solution::possible_bipartition(3, dislikes));

    let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    assert_eq!(false, Solution::possible_bipartition(5, dislikes));
}
