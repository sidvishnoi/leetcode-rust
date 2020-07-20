mod list_node;
use list_node::*;

struct Solution;
type Node = Option<Box<ListNode>>;
impl Solution {
    pub fn remove_elements(head: Node, val: i32) -> Node {
        let mut head = head;
        let mut curr = &mut head;
        while curr.is_some() {
            if curr.as_ref().unwrap().val == val {
                *curr = curr.take().unwrap().next;
            } else {
                curr = &mut curr.as_mut().unwrap().next;
            }
        }
        head
    }
}

fn main() {
    let list = vector_to_linked_list(vec![1, 2, 6, 3, 4, 5, 6]);
    assert_eq!(
        vec![1, 2, 3, 4, 5],
        linked_list_to_vector(Solution::remove_elements(list, 6))
    );
}
