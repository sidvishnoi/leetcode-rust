mod list_node;
use list_node::*;

struct Solution;
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_head = Some(Box::new(ListNode::new(-1)));
        let mut even_head = Some(Box::new(ListNode::new(-1)));

        let mut odd = &mut odd_head;
        let mut even = &mut even_head;

        let mut is_odd = true;
        let mut curr = head;
        while let Some(node) = curr {
            if is_odd {
                odd.as_mut().unwrap().next = Some(node);
                odd = &mut odd.as_mut().unwrap().next;
                curr = odd.as_mut().unwrap().next.take();
            } else {
                even.as_mut().unwrap().next = Some(node);
                even = &mut even.as_mut().unwrap().next;
                curr = even.as_mut().unwrap().next.take();
            }
            is_odd = !is_odd;
        }
        odd.as_mut().unwrap().next = even_head.unwrap().next;
        odd_head.unwrap().next
    }
}

fn main() {
    assert_eq!(
        vector_to_linked_list(vec![1, 3, 5, 2, 4]),
        Solution::odd_even_list(vector_to_linked_list(vec![1, 2, 3, 4, 5]))
    );
    assert_eq!(
        vector_to_linked_list(vec![2, 3, 6, 7, 1, 5, 4]),
        Solution::odd_even_list(vector_to_linked_list(vec![2, 1, 3, 5, 6, 4, 7]))
    );
}
