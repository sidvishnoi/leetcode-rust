mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Node {
        Self::from_preorder(&preorder[..])
    }

    fn from_preorder(preorder: &[i32]) -> Node {
        if preorder.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));

        let mut split_idx: usize = 1;
        while split_idx < preorder.len() && preorder[split_idx] < preorder[0] {
            split_idx += 1;
        }

        let left = &preorder[1..split_idx];
        let right = &preorder[split_idx..];
        root.clone().unwrap().borrow_mut().left = Self::from_preorder(left);
        root.clone().unwrap().borrow_mut().right = Self::from_preorder(right);
        root
    }
}

fn main() {
    assert_eq!(
        tree![8, 5, 10, 1, 7, null, 12],
        Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12])
    )
}
