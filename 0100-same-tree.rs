mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(ref p), Some(ref q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            _ => false,
        }
    }
}

fn main() {
    let p = tree![1, 2, 3];
    let q = tree![1, 2, 3];
    assert_eq!(true, Solution::is_same_tree(p, q));

    let p = tree![1, 2];
    let q = tree![1, null, 2];
    assert_eq!(false, Solution::is_same_tree(p, q));

    let p = tree![1, 2, 1];
    let q = tree![1, 1, 2];
    assert_eq!(false, Solution::is_same_tree(p, q));
}
