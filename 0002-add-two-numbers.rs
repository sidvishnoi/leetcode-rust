use utils::list_node::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut p, mut q) = (l1.as_ref(), l2.as_ref());
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut curr = &mut dummy_head;
    let mut carry = 0;
    while p.is_some() || q.is_some() {
        let x = p.map_or(0, |node| node.as_ref().val);
        let y = q.map_or(0, |node| node.as_ref().val);
        let sum = carry + x + y;
        carry = sum / 10;

        curr.next = Some(Box::new(ListNode::new(sum % 10)));

        curr = curr.next.as_mut().unwrap();
        p = p.map_or(None, |node| node.next.as_ref());
        q = q.map_or(None, |node| node.next.as_ref());
    }
    if carry != 0 {
        curr.next = Some(Box::new(ListNode::new(1)));
    }

    dummy_head.next
}

#[cfg(test)]
mod add_two_numbers {
    use super::add_two_numbers;
    use utils::list_node::{linked_list_to_vector, vector_to_linked_list};

    fn test(a: Vec<i32>, b: Vec<i32>, expected: Vec<i32>) {
        let l1 = vector_to_linked_list(a);
        let l2 = vector_to_linked_list(b);
        let sum = add_two_numbers(l1, l2);
        assert_eq!(linked_list_to_vector(sum), expected);
    }

    #[test]
    fn all_lengths_equal() {
        test(vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]);
        test(vec![1], vec![2], vec![3]);
        test(vec![], vec![], vec![]);
    }
    #[test]
    fn output_is_longer() {
        test(vec![2, 4, 7], vec![5, 6, 4], vec![7, 0, 2, 1]);
    }
    #[test]
    fn unequal_input_lengths() {
        test(vec![2], vec![5, 6, 4], vec![7, 6, 4]);
        test(vec![], vec![5, 6, 4], vec![5, 6, 4]);
        test(vec![5, 6, 4], vec![1, 0], vec![6, 6, 4]);
    }
}
