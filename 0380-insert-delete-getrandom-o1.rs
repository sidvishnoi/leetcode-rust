use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    index: HashMap<i32, usize>,
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.index.contains_key(&val) {
            return false;
        }
        self.index.insert(val, self.values.len());
        self.values.push(val);
        true
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.index.remove(&val) {
            let last = *self.values.last().unwrap();
            self.index
                .entry(last)
                .and_modify(|old_index| *old_index = idx);
            self.values[idx] = last;
            self.values.pop();
            return true;
        }
        false
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, self.values.len());
        self.values[idx]
    }
}

fn main() {
    let mut set = RandomizedSet::new();
    assert_eq!(true, set.insert(1));
    assert_eq!(false, set.remove(2));
    assert_eq!(true, set.insert(2));
    // assert_eq!(1, set.get_random());
    assert_eq!(true, set.remove(1));
    assert_eq!(false, set.insert(2));
    assert_eq!(2, set.get_random());
}
