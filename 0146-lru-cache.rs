use std::collections::HashMap;
use std::ptr;

pub struct ListNode {
    key: i32,
    value: i32,
    prev: *mut ListNode,
    next: *mut ListNode,
}

impl ListNode {
    pub fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

struct LRUMapList {
    store: HashMap<i32, Box<ListNode>>,
    head: Box<ListNode>,
    tail: Box<ListNode>,
}

impl LRUMapList {
    pub fn new() -> Self {
        let store = HashMap::new();
        let mut head = Box::new(ListNode::new(0, 0));
        let mut tail = Box::new(ListNode::new(0, 0));
        head.next = tail.as_mut();
        tail.prev = head.as_mut();
        Self { store, head, tail }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        self.update(key)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.update(key).is_some() {
            unsafe {
                (*self.tail.prev).value = value;
            }
        } else {
            let mut node = Box::new(ListNode::new(key, value));
            self.append(node.as_mut());
            self.store.insert(key, node);
        }
    }

    pub fn pop(&mut self) {
        if self.store.is_empty() {
            return;
        }

        let key = unsafe { (*self.head.next).key };
        if let Some(node) = self.store.remove(&key).as_mut() {
            self.detach(node.as_mut());
        }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    fn append(&mut self, node: *mut ListNode) {
        unsafe {
            (*self.tail.prev).next = node;
            (*node).prev = self.tail.prev;
            self.tail.prev = node;
            (*node).next = self.tail.as_mut();
        }
    }

    fn detach(&mut self, node: *mut ListNode) {
        unsafe {
            let curr = &*node;
            (*curr.prev).next = curr.next;
            (*curr.next).prev = curr.prev;
        }
    }

    fn update(&mut self, key: i32) -> Option<i32> {
        if let Some(node) = self.store.get_mut(&key) {
            let node = node.as_mut() as *mut ListNode;
            let value = unsafe { (*node).value };
            self.detach(node);
            self.append(node);
            return Some(value);
        }
        None
    }
}

struct LRUCache {
    capacity: usize,
    values: LRUMapList,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            values: LRUMapList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.values.get(key).unwrap_or(-1)
    }

    fn put(&mut self, key: i32, value: i32) {
        self.values.put(key, value);
        if self.values.len() > self.capacity {
            self.values.pop();
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(1, cache.get(1));
    cache.put(3, 3); // evicts key 2
    assert_eq!(-1, cache.get(2));
    cache.put(4, 4); // evicts key 1
    assert_eq!(-1, cache.get(1));
    assert_eq!(3, cache.get(3));
    assert_eq!(4, cache.get(4));
}
