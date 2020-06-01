mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(ref r) => {
                let left = Solution::invert_tree(r.borrow().left.clone());
                let right = Solution::invert_tree(r.borrow().right.clone());
                r.borrow_mut().right = left;
                r.borrow_mut().left = right;
                root
            }
            None => None,
        }
    }
}

fn main() {
    assert_eq!(
        tree![4, 7, 2, 9, 6, 3, 1],
        Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9])
    );
}

// public TreeNode invertTree(TreeNode root) {
//     if (root == null) return null;
//     TreeNode right = invertTree(root.right);
//     TreeNode left = invertTree(root.left);
//     root.left = right;
//     root.right = left;
//     return root;
// }
