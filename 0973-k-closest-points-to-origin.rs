struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_by_cached_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.resize(k as usize, vec![]);
        points
    }
}

fn main() {
    let points = vec![vec![1, 3], vec![-2, 2]];
    assert_eq!(vec![vec![-2, 2]], Solution::k_closest(points, 1));

    let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
    assert_eq!(
        vec![vec![3, 3], vec![-2, 4]],
        Solution::k_closest(points, 2)
    );
}
