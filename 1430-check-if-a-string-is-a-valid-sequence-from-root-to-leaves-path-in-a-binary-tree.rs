mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        Self::is_valid(root, &arr, 0_usize)
    }

    fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, arr: &Vec<i32>, i: usize) -> bool {
        if i >= arr.len() {
            return false;
        }
        match root {
            None => arr.len() == 0,
            Some(ref r) => {
                let is_leaf = r.borrow().left.is_none() && r.borrow().right.is_none();
                if is_leaf && i == arr.len() - 1 && r.borrow().val == arr[i] {
                    return true;
                }
                let left = r.borrow().left.as_ref().map_or(None, |x| Some(x.clone()));
                let right = r.borrow().right.as_ref().map_or(None, |x| Some(x.clone()));

                r.borrow().val == arr[i]
                    && (Self::is_valid(left, arr, i + 1) || Self::is_valid(right, arr, i + 1))
            }
        }
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::is_valid_sequence(
            tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0],
            vec![0, 1, 0, 1]
        )
    );
    assert_eq!(
        false,
        Solution::is_valid_sequence(tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0], vec![0, 0, 1])
    );
    assert_eq!(
        false,
        Solution::is_valid_sequence(tree![0, 1, 0, 0, 1, 0, null, null, 1, 0, 0], vec![0, 1, 1])
    );
}
