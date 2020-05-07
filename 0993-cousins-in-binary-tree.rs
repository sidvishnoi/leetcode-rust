mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut found_x = false;
            let mut found_y = false;
            for _ in 0..queue.len() {
                let curr = queue.pop_front().unwrap();

                if curr.borrow().val == x {
                    found_x = true;
                } else if curr.borrow().val == y {
                    found_y = true;
                }

                if curr.borrow().left.is_some() && curr.borrow().right.is_some() {
                    let left = curr.borrow().left.as_ref().unwrap().borrow().val;
                    let right = curr.borrow().right.as_ref().unwrap().borrow().val;
                    if (left == x && right == y) || (left == y && right == x) {
                        // they've same parent
                        return false;
                    }
                }
                if curr.borrow().left.is_some() {
                    queue.push_back(Rc::clone(curr.borrow().left.as_ref().unwrap()));
                }
                if curr.borrow().right.is_some() {
                    queue.push_back(Rc::clone(curr.borrow().right.as_ref().unwrap()));
                }
            }
            if found_x && found_y {
                // found both x and y on same level
                return true;
            } else if found_x || found_y {
                // found either x or y, but not both, i.e., they're not on same level
                return false;
            }
        }
        false
    }
}

fn main() {
    assert_eq!(false, Solution::is_cousins(tree![1, 2, 3, 4], 4, 3));
    assert_eq!(
        true,
        Solution::is_cousins(tree![1, 2, 3, null, 4, null, 5], 5, 4)
    );
    assert_eq!(false, Solution::is_cousins(tree![1, 2, 3, null, 4], 2, 3));
}
