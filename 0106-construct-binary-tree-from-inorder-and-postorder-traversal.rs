mod tree;
use tree::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }

        // Optimization: Get index of a value in inorder traversal in O(1) time.
        let inorder_indexes: HashMap<i32, i32> = inorder
            .iter()
            .enumerate()
            .map(|(idx, &val)| (val, idx as i32))
            .collect();

        Self::build(
            0,
            inorder.len() as i32 - 1,
            postorder.len() as i32 - 1,
            &inorder,
            &postorder,
            &inorder_indexes,
        )
    }

    fn build(
        in_start: i32,
        in_end: i32,
        post_start: i32,
        inorder: &Vec<i32>,
        postorder: &Vec<i32>,
        inorder_indexes: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start > in_end || post_start < 0 {
            return None;
        }

        let val = postorder[post_start as usize];
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(val))));

        let mid = *inorder_indexes.get(&val).unwrap();
        root.as_mut().unwrap().borrow_mut().left = Self::build(
            in_start,
            mid - 1,
            post_start - (in_end - mid) - 1,
            inorder,
            postorder,
            inorder_indexes,
        );
        root.as_mut().unwrap().borrow_mut().right = Self::build(
            mid + 1,
            in_end,
            post_start - 1,
            inorder,
            postorder,
            inorder_indexes,
        );
        root
    }
}

fn main() {
    assert_eq!(
        tree![3, 9, 20, null, null, 15, 7],
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}
