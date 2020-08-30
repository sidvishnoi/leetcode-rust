struct Solution;

use std::{cmp, collections::HashMap};

#[derive(Debug)]
struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x) as usize;
        let root_y = self.find(y) as usize;
        if root_x != root_y {
            self.parent[root_x] = self.parent[root_y];
        }
    }

    fn find(&mut self, mut x: i32) -> i32 {
        let mut root = x;
        while root != self.parent[root as usize] {
            root = self.parent[root as usize];
        }
        while x != root {
            let p = self.parent[x as usize];
            self.parent[x as usize] = root;
            x = p;
        }
        root
    }
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let max = *a.iter().max().unwrap();
        let mut union_find = UnionFind::new(max + 1);
        for &num in &a {
            for k in 2..=((num as f64).sqrt() as i32) {
                if num % k == 0 {
                    union_find.union(num, k);
                    union_find.union(num, num / k);
                }
            }
        }

        let mut sizes = HashMap::new();
        let mut max_component_size = 1;
        for num in a {
            let root = union_find.find(num);
            let component_size = *sizes.entry(root).and_modify(|v| *v += 1).or_insert(1);
            max_component_size = cmp::max(max_component_size, component_size);
        }
        max_component_size
    }
}

fn main() {
    assert_eq!(4, Solution::largest_component_size(vec![4, 6, 15, 35]));
    assert_eq!(2, Solution::largest_component_size(vec![20, 50, 9, 63]));
    assert_eq!(
        8,
        Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39])
    );
}
