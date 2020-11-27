use utils::list_node::ListNode;

type Node = Option<Box<ListNode>>;
pub fn merge_two_lists(l1: Node, l2: Node) -> Node {
    let mut result = Box::new(ListNode::new(0));
    let (mut p, mut q) = (l1.as_ref(), l2.as_ref());
    let mut iter = &mut result;
    while p.is_some() && q.is_some() {
        let (a, b) = (p.unwrap(), q.unwrap());
        if a.val < b.val {
            iter.next = p.cloned();
            p = a.next.as_ref();
        } else {
            iter.next = q.cloned();
            q = b.next.as_ref();
        }
        iter = iter.next.as_mut()?;
    }
    iter.next = if p.is_some() { p.cloned() } else { q.cloned() };
    println!("{:p}", result.next?);
    println!("{:p}", l1?);
    println!("{:p}", l2?);
    None
    // result.next
}

#[cfg(test)]
mod merge_two_lists {
    use super::merge_two_lists;
    use utils::list_node::{linked_list_to_vector, vector_to_linked_list};

    fn test(a: Vec<i32>, b: Vec<i32>, expected: Vec<i32>) {
        let l1 = vector_to_linked_list(a);
        let l2 = vector_to_linked_list(b);
        let merged = merge_two_lists(l1, l2);
        assert_eq!(linked_list_to_vector(merged), expected);
    }

    #[test]
    fn it_works() {
        test(vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4]);
        test(vec![2], vec![1], vec![1, 2]);
        test(vec![], vec![], vec![]);
    }
}
