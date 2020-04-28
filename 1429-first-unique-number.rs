use std::collections::HashMap;
use std::ptr;

pub struct ListNode {
    value: i32,
    prev: *mut ListNode,
    next: *mut ListNode,
}

impl ListNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

struct List {
    length: usize,
    head: Box<ListNode>,
    tail: Box<ListNode>,
}

impl List {
    pub fn new() -> Self {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = Box::new(ListNode::new(0));
        head.next = tail.as_mut();
        tail.prev = head.as_mut();
        let length = 0_usize;
        Self { head, tail, length }
    }

    pub fn append(&mut self, node: *mut ListNode) {
        unsafe {
            (*self.tail.prev).next = node;
            (*node).prev = self.tail.prev;
            self.tail.prev = node;
            (*node).next = self.tail.as_mut();
        }
        self.length += 1;
    }

    pub fn remove(&mut self, node: *mut ListNode) {
        unsafe {
            let curr = &*node;
            (*curr.prev).next = curr.next;
            (*curr.next).prev = curr.prev;
        }
        self.length -= 1;
    }

    pub fn front(&self) -> Option<i32> {
        match self.length {
            0 => None,
            _ => unsafe { Some((*self.head.next).value) },
        }
    }
}

struct FirstUnique {
    store: HashMap<i32, Option<Box<ListNode>>>,
    unique: List,
}

impl FirstUnique {
    pub fn new(nums: Vec<i32>) -> Self {
        let store = HashMap::new();
        let unique = List::new();
        let mut s = Self { store, unique };
        for num in nums {
            Self::add(&mut s, num);
        }
        s
    }

    pub fn show_first_unique(&mut self) -> i32 {
        self.unique.front().unwrap_or(-1)
    }

    pub fn add(&mut self, value: i32) {
        match self.store.get_mut(&value) {
            None => {
                let mut node = Box::new(ListNode::new(value));
                self.unique.append(node.as_mut());
                self.store.insert(value, Some(node));
            }
            Some(None) => {}
            Some(node) => {
                // mark the value in hashmap as already taken, so we don't try to remove it again
                let mut node = node.take().unwrap();
                let node_ptr = node.as_mut() as *mut ListNode;
                self.unique.remove(node_ptr);
            }
        }
    }
}

fn main() {
    {
        let mut first_unique = FirstUnique::new(vec![2, 3, 5]);
        assert_eq!(2, first_unique.show_first_unique()); // return 2
        first_unique.add(5); // the queue is now [2,3,5,5]
        assert_eq!(2, first_unique.show_first_unique()); // return 2
        first_unique.add(2); // the queue is now [2,3,5,5,2]
        assert_eq!(3, first_unique.show_first_unique()); // return 3
        first_unique.add(3); // the queue is now [2,3,5,5,2,3]
        assert_eq!(-1, first_unique.show_first_unique()); // return -1
    }
    println!();
    {
        let mut first_unique = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
        assert_eq!(-1, first_unique.show_first_unique()); // return -1
        first_unique.add(7); // the queue is now [7,7,7,7,7,7,7]
        first_unique.add(3); // the queue is now [7,7,7,7,7,7,7,3]
        first_unique.add(3); // the queue is now [7,7,7,7,7,7,7,3,3]
        first_unique.add(7); // the queue is now [7,7,7,7,7,7,7,3,3,7]
        first_unique.add(17); // the queue is now [7,7,7,7,7,7,7,3,3,7,17]
        assert_eq!(17, first_unique.show_first_unique()); // return 17
    }
    println!();
    {
        let mut first_unique = FirstUnique::new(vec![809]);
        assert_eq!(809, first_unique.show_first_unique()); // return 809
        first_unique.add(809); // the queue is now [809,809]
        assert_eq!(-1, first_unique.show_first_unique()); // return -1
    }
    println!();
    {
        let mut first_unique = FirstUnique::new(vec![]);
        assert_eq!(-1, first_unique.show_first_unique()); // return -1
        assert_eq!(-1, first_unique.show_first_unique()); // return -1
        first_unique.add(809); // the queue is now [809]
        assert_eq!(809, first_unique.show_first_unique()); // return 809
    }
}
