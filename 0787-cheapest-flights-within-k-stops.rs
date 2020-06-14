struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut prices: HashMap<i32, HashMap<i32, i32>> = HashMap::with_capacity(n as usize);
        for flight in flights {
            let (u, v, w) = (flight[0], flight[1], flight[2]);
            prices.entry(u).or_default().entry(v).or_insert(w);
        }
        // We want a min-heap
        let mut queue: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        queue.push(Reverse((0, src, k + 1)));
        while let Some(Reverse((price, city, allowed_stops))) = queue.pop() {
            if city == dst {
                return price;
            }
            if allowed_stops > 0 {
                for (neighbour, p) in prices.get(&city).unwrap_or(&HashMap::new()).iter() {
                    queue.push(Reverse((price + p, *neighbour, allowed_stops - 1)));
                }
            }
        }
        -1
    }
}

fn main() {
    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(200, Solution::find_cheapest_price(3, flights, 0, 2, 1));

    let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    assert_eq!(500, Solution::find_cheapest_price(3, flights, 0, 2, 0));
}
