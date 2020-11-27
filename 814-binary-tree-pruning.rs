mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(ref r) => {
                let left = r.borrow().left.as_ref().map_or(None, |x| Some(x.clone()));
                r.borrow_mut().left = Self::prune_tree(left);

                let right = r.borrow().right.as_ref().map_or(None, |x| Some(x.clone()));
                r.borrow_mut().right = Self::prune_tree(right);

                if r.borrow().val == 0 && r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return None;
                }
                root
            }
        }
    }
}

fn main() {
    // let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // let right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    // right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    // right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.as_ref().unwrap().borrow_mut().right = right;
    let root = tree![1, null, 0, 0, 1];
    println!("{:#?}", root);
    let result = Solution::prune_tree(root);
    println!("{:#?}", result);
}

// ``` java
// public TreeNode pruneTree(TreeNode root) {
//     if ( root == null ) return null;

//     root.left = pruneTree (root.left);
//     root.right = pruneTree (root.right);

//     if ( root.val == 0 && root.left == null && root.right == null ) return null;
//     return root;
// }
// ```
