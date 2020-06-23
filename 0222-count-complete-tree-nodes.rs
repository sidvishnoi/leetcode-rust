mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(ref r) => {
                let mut left = root.clone();
                let mut right = root.clone();
                let mut height = 0;
                while right.is_some() {
                    left = left.unwrap().borrow().left.clone();
                    right = right.unwrap().borrow().right.clone();
                    height += 1;
                }
                if left.is_none() {
                    // right.is_none() already, so we have a full tree, i.e.,
                    // (2^h - 1) nodes
                    return (1 << height) - 1;
                }

                let left = r.borrow().left.clone();
                let right = r.borrow().right.clone();
                1 + Self::count_nodes(left) + Self::count_nodes(right)
            }
            None => 0,
        }
    }
}

fn main() {
    assert_eq!(6, Solution::count_nodes(tree![1, 2, 3, 4, 5, 6]));
}
