mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut root = root;
        let mut k = k;
        loop {
            while let Some(r) = root {
                stack.push(Some(r.clone()));
                root = r.borrow().left.as_ref().map_or(None, |x| Some(x.clone()));
            }
            root = stack.pop().unwrap().clone();
            k -= 1;
            let r = root.as_ref().unwrap();
            if k == 0 {
                return r.borrow().val;
            }
            root = r
                .clone()
                .borrow()
                .right
                .as_ref()
                .map_or(None, |x| Some(x.clone()));
        }
    }
}

fn main() {
    let root = tree![3, 1, 4, null, 2];
    assert_eq!(1, Solution::kth_smallest(root, 1));

    let root = tree![5, 3, 6, 2, 4, null, null, 1];
    assert_eq!(3, Solution::kth_smallest(root, 3));
}
