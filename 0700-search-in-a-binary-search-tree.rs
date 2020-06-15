mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn search_bst(root: Node, val: i32) -> Node {
        let mut root = root;
        while let Some(r) = root {
            if r.borrow().val > val {
                root = r.borrow().left.clone();
            } else if r.borrow().val < val {
                root = r.borrow().right.clone();
            } else {
                return Some(r);
            }
        }
        None
    }
}

fn main() {
    assert_eq!(
        tree![2, 1, 3],
        Solution::search_bst(tree![4, 2, 7, 1, 3], 2)
    )
}
