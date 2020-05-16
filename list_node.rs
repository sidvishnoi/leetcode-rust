#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vector_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut iter = &mut dummy_head;
    for num in nums {
        iter.next = Some(Box::new(ListNode::new(num)));
        iter = iter.next.as_mut().unwrap();
    }
    dummy_head.next
}

pub fn linked_list_to_vector(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut iter = list.as_ref();
    while let Some(node) = iter {
        result.push(node.val);
        iter = node.next.as_ref();
    }
    result
}
