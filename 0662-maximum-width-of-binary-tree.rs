mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn width_of_binary_tree(root: Node) -> i32 {
        let mut leftmost_index_by_level = Vec::new();
        Self::dfs(root, 0, 1, &mut leftmost_index_by_level)
    }

    fn dfs(root: Node, level: usize, index: i32, lefts: &mut Vec<i32>) -> i32 {
        match root {
            None => 0,
            Some(ref r) => {
                if lefts.len() == level {
                    lefts.push(index);
                }
                let current_width = index - lefts[level] + 1;
                let left = Self::dfs(r.borrow().left.clone(), level + 1, index * 2 + 1, lefts);
                let right = Self::dfs(r.borrow().right.clone(), level + 1, index * 2 + 2, lefts);
                std::cmp::max(current_width, std::cmp::max(left, right))
            }
        }
    }
}

fn main() {
    assert_eq!(
        4,
        Solution::width_of_binary_tree(tree![1, 3, 2, 5, 3, null, 9])
    );
    assert_eq!(2, Solution::width_of_binary_tree(tree![1, 3, null, 5, 3]));
    assert_eq!(2, Solution::width_of_binary_tree(tree![1, 3, 2, 5]));
    assert_eq!(
        8,
        Solution::width_of_binary_tree(tree![1, 3, 2, 5, null, null, 9, 6, null, null, 7])
    );
}
