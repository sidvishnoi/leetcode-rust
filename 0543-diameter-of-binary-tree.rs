mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root).0
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(ref r) => {
                let left_node = r.borrow().left.as_ref().map_or(None, |x| Some(x.clone()));
                let right_node = r.borrow().right.as_ref().map_or(None, |x| Some(x.clone()));

                let (left_diameter, left_height) = Self::dfs(left_node);
                let (right_diameter, right_height) = Self::dfs(right_node);

                let diameter = std::cmp::max(
                    left_height + right_height,
                    std::cmp::max(left_diameter, right_diameter),
                );
                let height = 1 + std::cmp::max(left_height, right_height);
                (diameter, height)
            }
            None => (0, 0),
        }
    }
}

fn main() {
    assert_eq!(3, Solution::diameter_of_binary_tree(tree![1, 2, 3, 4, 5]));
}
