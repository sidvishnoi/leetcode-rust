mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut sum = 0;
        if let Some(ref left) = root.as_ref().unwrap().borrow().left {
            if left.borrow().left.is_none() && left.borrow().right.is_none() {
                sum += left.borrow().val;
            } else {
                sum += Self::sum_of_left_leaves(Some(left.clone()));
            }
        }
        sum += Self::sum_of_left_leaves(root.unwrap().borrow().right.clone());

        sum
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
