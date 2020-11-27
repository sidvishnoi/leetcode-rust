struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Position = (usize, usize);
type Grid<T> = Vec<Vec<T>>;
type Path = Vec<Position>;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    priority: i32,
    position: Position,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct AStar {
    grid: Grid<i32>,
}

impl AStar {
    pub fn new(grid: Grid<i32>) -> Self {
        Self { grid }
    }

    pub fn find_path(&self, start: Position, goal: Position) -> Option<Path> {
        let mut distances: HashMap<Position, i32> = HashMap::new();
        let mut visited: HashSet<Position> = HashSet::new();
        let mut frontier = BinaryHeap::new();

        frontier.push(Node {
            priority: 0,
            position: start,
        });
        distances.insert(start, 0);

        let mut came_from: HashMap<Position, Position> = HashMap::new();
        while let Some(node) = frontier.pop() {
            let current = node.position;
            if visited.contains(&current) {
                continue;
            }
            if current == goal {
                let path_followed = Self::reconstruct_path(start, goal, came_from);
                return Some(path_followed);
            }

            visited.insert(current);
            for next in self.get_successors(current) {
                let priority = distances[&current] + 1 + Self::heuristic(goal, next);
                frontier.push(Node {
                    priority,
                    position: next,
                });

                if !distances.contains_key(&next) || distances[&current] + 1 < distances[&next] {
                    distances.insert(next, distances[&current] + 1);
                    came_from.insert(next, current);
                }
            }
        }

        None
    }

    fn get_successors(&self, from: Position) -> Vec<Position> {
        let (m, n) = (self.grid.len(), self.grid[0].len());
        let (rr, cc) = from;

        let mut successors = Vec::new();
        for (dr, dc) in DIRECTIONS {
            let r = match (rr, dr) {
                (0, -1) => continue,
                _ => ((rr as i32) + dr) as usize,
            };
            let c = match (cc, dc) {
                (0, -1) => continue,
                _ => ((cc as i32) + dc) as usize,
            };
            if r >= m || c >= n || self.grid[r][c] == 1 {
                continue;
            }
            let successor: Position = (r, c);
            successors.push(successor);
        }
        successors
    }

    fn reconstruct_path(
        start: Position,
        goal: Position,
        came_from: HashMap<Position, Position>,
    ) -> Path {
        let mut current = goal;
        let mut reverse_path = vec![current];
        while current != start {
            current = came_from[&current];
            reverse_path.push(current);
        }
        let path_followed: Vec<_> = reverse_path.iter().cloned().rev().collect();
        path_followed
    }

    fn heuristic(goal: Position, pos: Position) -> i32 {
        let (r, c) = pos;
        let (m, n) = goal;
        std::cmp::max(m - r, n - c) as i32
    }

    pub fn print_path(&self, path: &Path) {
        let mut output: Vec<Vec<String>> = Vec::new();
        for row in &self.grid {
            let mut p = vec![];
            for col in row {
                p.push(format!("{} ", if *col == 1 { "⬛" } else { "🏼" }));
            }
            output.push(p);
        }
        for (r, c) in path {
            output[*r][*c] = String::from("💠 ");
        }
        for row in output {
            for col in row {
                print!("{}", col);
            }
            print!("\n");
        }
        // println!("{:?}", path);
    }
}

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Grid<i32>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if grid[0][0] == 1 || grid[m - 1][n - 1] == 1 {
            return -1;
        }

        let astar = AStar::new(grid);
        let start: Position = (0, 0);
        let goal: Position = (m - 1, n - 1);
        match astar.find_path(start, goal) {
            Some(path) => {
                astar.print_path(&path);
                path.len() as i32
            }
            None => -1,
        }
    }
}

fn main() {
    let grids = vec![
        vec![
            vec![0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 1, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0],
        ],
        vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]],
        vec![
            vec![0, 0, 1, 0, 1, 1],
            vec![1, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![
                0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0,
            ],
            vec![
                1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1,
            ],
            vec![
                0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            ],
            vec![
                0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1,
            ],
            vec![
                1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0,
            ],
            vec![
                0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1,
            ],
            vec![
                1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1,
            ],
            vec![
                0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            vec![
                1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1,
            ],
            vec![
                0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,
            ],
            vec![
                0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0,
            ],
            vec![
                0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1,
            ],
            vec![
                0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,
            ],
            vec![
                0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0,
            ],
            vec![
                1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1,
            ],
            vec![
                0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0,
            ],
            vec![
                0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0,
            ],
            vec![
                0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1,
            ],
            vec![
                0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0,
            ],
            vec![
                1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1,
            ],
            vec![
                1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            vec![
                0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1,
            ],
            vec![
                0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0,
            ],
        ],
    ];
    for grid in grids {
        let result = Solution::shortest_path_binary_matrix(grid);
        println!("result: {:?}", result);
    }
}
