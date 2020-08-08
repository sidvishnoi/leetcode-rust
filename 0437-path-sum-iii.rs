mod tree;
use tree::*;
struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        // Cache pathSum => countOfSuchPaths
        let mut cache: HashMap<i32, i32> = HashMap::new();
        cache.insert(0, 1);
        Self::dfs(root, 0, sum, &mut cache)
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut current_path_sum: i32,
        target: i32,
        cache: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(ref node) = root {
            current_path_sum += node.borrow().val;

            let old_path_sum = current_path_sum - target;
            let mut result = *cache.get(&old_path_sum).or(Some(&0)).unwrap();

            *cache.entry(current_path_sum).or_insert(0) += 1;

            result += Self::dfs(node.borrow().left.clone(), current_path_sum, target, cache)
                + Self::dfs(node.borrow().right.clone(), current_path_sum, target, cache);

            // when moving to a different branch, the current_path_sum is no
            // longer available, hence remove one such path.
            *cache.get_mut(&current_path_sum).unwrap() -= 1;

            result
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
    assert_eq!(0, Solution::path_sum(tree![1], 0));
}
