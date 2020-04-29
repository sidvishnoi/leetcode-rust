mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = std::i32::MIN;
        Self::helper(root, &mut result);
        result
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(ref r) => {
                let left_node = r.borrow().left.as_ref().map_or(None, |x| Some(x.clone()));
                let right_node = r.borrow().right.as_ref().map_or(None, |x| Some(x.clone()));

                let left_max = Self::helper(left_node, result);
                let right_max = Self::helper(right_node, result);
                let root_val = r.borrow().val;

                *result = std::cmp::max(*result, root_val + left_max + right_max);

                // max value with r as root
                std::cmp::max(0, root_val + std::cmp::max(left_max, right_max))
            }
        }
    }
}

fn main() {
    assert_eq!(6, Solution::max_path_sum(tree![1, 2, 3]));
    assert_eq!(
        42,
        Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7])
    );
}
