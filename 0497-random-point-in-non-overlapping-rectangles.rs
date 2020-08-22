use rand::distributions::{Distribution, Uniform};
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

struct Solution {
    rects: Vec<Vec<i32>>,
    map: BTreeMap<i32, usize>,
    total_points: i32,
    get_random_point_count: Box<dyn FnMut() -> i32>,
}

// randomly pick a rectangle weighted by its area,
// then randomly pick a point in that rectangle.
impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut map = BTreeMap::new();
        let mut total_points = 0;
        for (i, rect) in rects.iter().enumerate() {
            if let [x1, y1, x2, y2] = &rect[..] {
                let num_points_in_rect = (x2 - x1 + 1) * (y2 - y1 + 1);
                total_points += num_points_in_rect;
                map.insert(total_points, i);
            }
        }

        let mut rng = rand::thread_rng();
        let sampler = Uniform::from(0..total_points);
        let get_random_point_count = Box::new(move || sampler.sample(&mut rng));

        Self {
            rects,
            map,
            total_points,
            get_random_point_count,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let random_count = (self.get_random_point_count)();
        let (count, i) = self.first_random(random_count);
        let random_rect = &self.rects[i];

        if let [x1, y1, x2, _y2] = &random_rect[..] {
            // following is an optimized way to find x = rand(x1, x2) and y = rand(y1, y2)
            let (random_area, width) = (count - random_count - 1, x2 - x1 + 1);
            let x = x1 + random_area % width;
            let y = y1 + random_area / width;
            vec![x, y]
        } else {
            unreachable!()
        }
    }

    // find the first map entry with point_count >= count
    fn first_random(&self, count: i32) -> (i32, usize) {
        let range = (Excluded(count), Included(self.total_points));
        let (&count, &i) = self.map.range(range).next().unwrap();
        (count, i)
    }
}

fn main() {
    let rects = vec![vec![1, 1, 5, 5]];
    let mut s = Solution::new(rects);
    for _ in 0..10 {
        let picked = s.pick();
        println!("{:?}", picked);
    }
    println!("");

    let rects = vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]];
    let mut s = Solution::new(rects);
    for _ in 0..10 {
        let picked = s.pick();
        println!("{:?}", picked);
    }
    println!("");

    let rects = vec![vec![-2, -2, -1, -1], vec![1, 0, 3, 0]];
    let mut s = Solution::new(rects);
    for _ in 0..20 {
        let picked = s.pick();
        println!("{:?}", picked);
    }
}
