struct MinStack {
    min: i32,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            min: std::i32::MAX,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if x <= self.min {
            self.stack.push(self.min);
            self.min = x;
        }
        self.stack.push(x);
    }

    fn pop(&mut self) {
        let popped = self.stack.pop().unwrap();
        if popped == self.min {
            self.min = self.stack.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    assert_eq!(-3, stack.get_min());
    stack.pop();
    assert_eq!(0, stack.top());
    assert_eq!(-2, stack.get_min());
}
