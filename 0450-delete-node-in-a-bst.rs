mod tree;
use tree::*;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn delete_node(mut root: Node, key: i32) -> Node {
        let (parent, node) = Self::find(root.clone(), key);

        if node.is_none() {
            // key not found
            return root;
        }

        let replacement = Self::delete_node_at(&node);
        if let Some(p) = parent {
            if node == p.borrow().left {
                p.borrow_mut().left = replacement;
            } else {
                p.borrow_mut().right = replacement;
            }
        } else {
            root = replacement;
        }
        root
    }

    fn find(root: Node, key: i32) -> (Node, Node) {
        let mut parent = None;
        let mut current = root;
        while let Some(curr) = current {
            if curr.borrow().val > key {
                current = curr.borrow().left.clone();
            } else if curr.borrow().val < key {
                current = curr.borrow().right.clone();
            } else {
                current = Some(curr);
                break;
            }
            parent = Some(curr);
        }
        (parent, current)
    }

    fn delete_node_at(root: &Node) -> Node {
        let root = match root {
            None => return None,
            Some(r) => r,
        };

        if root.borrow().left.is_none() {
            return root.borrow().right.clone();
        }
        if root.borrow().right.is_none() {
            return root.borrow().left.clone();
        }

        let mut parent = None;
        let mut next = root.borrow().right.clone();
        while next.as_ref().unwrap().borrow().left.is_some() {
            parent = next.clone();
            next = next.unwrap().borrow().left.clone();
        }
        next.as_ref().unwrap().borrow_mut().left = root.borrow().left.clone();
        if root.borrow().right != next {
            parent.unwrap().borrow_mut().left = next.as_ref().unwrap().borrow().right.clone();
            next.as_ref().unwrap().borrow_mut().right = root.borrow().right.clone();
        }
        next
    }
}

fn main() {
    assert_eq!(
        tree![5, 4, 6, 2, null, null, 7],
        Solution::delete_node(tree![5, 3, 6, 2, 4, null, 7], 3)
    );
}
