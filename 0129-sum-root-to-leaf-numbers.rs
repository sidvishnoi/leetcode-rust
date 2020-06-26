mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_number(root, 0)
    }

    // if number upto current node (root) is 123, then level_base is 120
    fn sum_number(root: Option<Rc<RefCell<TreeNode>>>, level_base: i32) -> i32 {
        if let Some(ref root) = root {
            let left = root.borrow().left.clone();
            let right = root.borrow().right.clone();
            if left.is_none() && right.is_none() {
                return level_base + root.borrow().val;
            }
            let next_level_base = (level_base + root.borrow().val) * 10;
            let left_sum = Self::sum_number(left, next_level_base);
            let right_sum = Self::sum_number(right, next_level_base);
            return left_sum + right_sum;
        }
        0
    }
}

fn main() {
    assert_eq!(25, Solution::sum_numbers(tree![1, 2, 3]));
    assert_eq!(1026, Solution::sum_numbers(tree![4, 9, 0, 5, 1]));
}
