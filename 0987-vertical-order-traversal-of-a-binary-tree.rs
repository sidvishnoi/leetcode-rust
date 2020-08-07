mod tree;
use tree::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

use std::collections::{BTreeMap, BTreeSet};

type Node = Option<Rc<RefCell<TreeNode>>>;
type NodeMap = BTreeMap<(i32, i32), BTreeSet<i32>>;

impl Solution {
    pub fn vertical_traversal(root: Node) -> Vec<Vec<i32>> {
        let mut node_map: NodeMap = BTreeMap::new();
        Self::traverse(root, 0, 0, &mut node_map);

        let mut result = Vec::new();
        let mut prev_x = std::i32::MIN;
        for ((x, _y), nodes) in node_map {
            if x != prev_x {
                result.push(Vec::new());
                prev_x = x;
            }
            result.last_mut().unwrap().extend(nodes);
        }
        result
    }

    fn traverse(root: Node, x: i32, y: i32, node_map: &mut NodeMap) {
        if let Some(ref r) = root {
            node_map.entry((x, y)).or_default().insert(r.borrow().val);
            Self::traverse(r.borrow().left.clone(), x - 1, y + 1, node_map);
            Self::traverse(r.borrow().right.clone(), x + 1, y + 1, node_map);
        }
    }
}

fn main() {
    assert_eq!(
        vec![vec![9], vec![3, 15], vec![20], vec![7]],
        Solution::vertical_traversal(tree![3, 9, 20, null, null, 15, 7])
    );
    assert_eq!(
        vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
        Solution::vertical_traversal(tree![1, 2, 3, 4, 5, 6, 7])
    );
    assert_eq!(
        vec![
            vec![4, 10, 11],
            vec![3, 6, 7],
            vec![2, 5, 8, 9],
            vec![0],
            vec![1]
        ],
        Solution::vertical_traversal(tree![
            0, 2, 1, 3, null, null, null, 4, 5, null, 7, 6, null, 10, 8, 11, 9
        ])
    );
}
