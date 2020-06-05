use rand::Rng;

struct Solution {
    cumulative_weights: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut cumulative_weights = w;
        for i in 1..cumulative_weights.len() {
            cumulative_weights[i] += cumulative_weights[i - 1];
        }
        Self { cumulative_weights }
    }

    fn pick_index(&self) -> i32 {
        let last = self.cumulative_weights.last().unwrap();
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0, last) + 1;

        let mut low = 0 as usize;
        let mut high = self.cumulative_weights.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            if self.cumulative_weights[mid] == r {
                return mid as i32;
            } else if self.cumulative_weights[mid] < r {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}

fn main() {
    let picker = Solution::new(vec![1, 3]);
    let mut counts = vec![0; 2];
    for _ in 0..10000 {
        let idx = picker.pick_index();
        counts[idx as usize] += 1;
    }
    println!("{:?}", counts);
}
