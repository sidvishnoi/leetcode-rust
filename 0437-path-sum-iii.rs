mod tree;
use tree::*;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(ref r) = root {
            Self::path_sum_from(root.clone(), sum)
                + Self::path_sum(r.borrow().left.clone(), sum)
                + Self::path_sum(r.borrow().right.clone(), sum)
        } else {
            0
        }
    }

    fn path_sum_from(root: Option<Rc<RefCell<TreeNode>>>, remaining_sum: i32) -> i32 {
        if let Some(ref node) = root {
            let val = node.borrow().val;
            let path_count = if val == remaining_sum { 1 } else { 0 };
            path_count
                + Self::path_sum_from(node.borrow().left.clone(), remaining_sum - val)
                + Self::path_sum_from(node.borrow().right.clone(), remaining_sum - val)
        } else {
            0
        }
    }
}

fn main() {
    assert_eq!(
        3,
        Solution::path_sum(tree![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1], 8)
    );
    assert_eq!(
        2,
        Solution::path_sum(tree![1, null, 2, null, 3, null, 4, null, 5], 3)
    );
}
