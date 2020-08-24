mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root, false)
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match root {
            Some(ref r) => {
                let left = r.borrow().left.clone();
                let right = r.borrow().right.clone();
                if left.is_none() && right.is_none() {
                    return match is_left {
                        true => r.borrow().val,
                        false => 0,
                    };
                }
                Self::helper(left, true) + Self::helper(right, false)
            }
            None => 0,
        }
    }
}

fn main() {
    assert_eq!(
        24,
        Solution::sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7])
    );
    assert_eq!(0, Solution::sum_of_left_leaves(tree![]));
    assert_eq!(0, Solution::sum_of_left_leaves(tree![1]));
    assert_eq!(2, Solution::sum_of_left_leaves(tree![1, 2, 3]));
}
