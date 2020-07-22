mod tree;
use tree::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut rtl = false;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.clone().unwrap());
        while !queue.is_empty() {
            let mut level = Vec::new();
            if rtl {
                // pop_front, push_back, right then left
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_front() {
                        level.push(node.borrow().val);
                        if let Some(right) = node.borrow().right.as_ref() {
                            queue.push_back(right.clone());
                        }
                        if let Some(left) = node.borrow().left.as_ref() {
                            queue.push_back(left.clone());
                        }
                    }
                }
            } else {
                // pop_back, push_front, left then right
                for _ in 0..queue.len() {
                    if let Some(node) = queue.pop_back() {
                        level.push(node.borrow().val);
                        if let Some(left) = node.borrow().left.as_ref() {
                            queue.push_front(left.clone());
                        }
                        if let Some(right) = node.borrow().right.as_ref() {
                            queue.push_front(right.clone());
                        }
                    }
                }
            }
            result.push(level);
            rtl = !rtl;
        }

        result
    }
}

fn main() {
    assert_eq!(
        vec![vec![3], vec![20, 9], vec![15, 7]],
        Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7])
    );
}
