mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let mut nodes = vec![vec![root.unwrap().clone()]];
        loop {
            let mut level = Vec::new();
            for node in nodes.last().unwrap() {
                if let Some(left) = node.borrow().left.clone() {
                    level.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    level.push(right);
                }
            }
            if level.is_empty() {
                break;
            }
            nodes.push(level);
        }

        nodes
            .into_iter()
            .rev()
            .map(|level| level.into_iter().map(|node| node.borrow().val).collect())
            .collect()
    }
}

fn main() {
    assert_eq!(
        vec![vec![15, 7], vec![9, 20], vec![3]],
        Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7])
    );
}
