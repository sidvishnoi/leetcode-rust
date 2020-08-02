struct MyHashSet {
    // We ideally want a Vec<LinkedList> here, but as Rust's LinkedList removal
    // is O(n), so we use a Vec anyway
    store: Vec<Vec<i32>>,
    count: usize,
    capacity: usize,
    load_factor: f64,
}

impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        let capacity = 1000_usize;
        Self {
            store: vec![Vec::new(); capacity],
            count: 0,
            capacity,
            load_factor: 0.75_f64,
        }
    }

    pub fn add(&mut self, key: i32) {
        if self.contains(key) {
            return;
        }

        if (self.load_factor * self.count as f64) as usize == self.capacity {
            self.count = 0;
            self.capacity *= 2;
            let old_store = self.store.clone();
            self.store = vec![Vec::new(); self.capacity];
            for bucket in old_store {
                for item in bucket {
                    self.add_item(item);
                }
            }
        }

        self.add_item(key);
    }

    fn add_item(&mut self, key: i32) {
        let bucket_id = self.hash(key);
        let bucket = self.store.get_mut(bucket_id).unwrap();
        bucket.push(key);
        self.count += 1;
    }

    pub fn remove(&mut self, key: i32) {
        let bucket_id = self.hash(key);
        let bucket = self.store.get_mut(bucket_id).unwrap();
        if let Some(idx) = bucket.iter().position(|x| *x == key) {
            bucket.remove(idx);
        }
    }

    /** Returns true if this set contains the specified element */
    pub fn contains(&self, key: i32) -> bool {
        let bucket = &self.store[self.hash(key)];
        bucket.contains(&key)
    }

    #[inline]
    fn hash(&self, key: i32) -> usize {
        key as usize % self.capacity
    }
}

fn main() {
    let mut set = MyHashSet::new();
    set.add(1);
    set.add(2);
    assert_eq!(true, set.contains(1));
    assert_eq!(false, set.contains(3));
    set.add(2);
    assert_eq!(true, set.contains(2));
    set.remove(2);
    assert_eq!(false, set.contains(2));
}
