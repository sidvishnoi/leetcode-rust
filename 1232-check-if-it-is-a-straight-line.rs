struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // compare slopes of all points with slope of any two points (here, first two)
        let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
        let (x1, y1) = (coordinates[1][0], coordinates[1][1]);

        let dx = x1 - x0;
        let dy = y1 - y0;
        for coord in coordinates.into_iter().skip(2) {
            let (x, y) = (coord[0], coord[1]);
            // check if dy/dx == (y-y1)/(x-x1), but avoid possible division by zero
            if dx * (y - y1) != dy * (x - x1) {
                return false;
            }
        }
        true
    }
}

fn main() {
    assert_eq!(
        true,
        Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ])
    );
    assert_eq!(
        false,
        Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ])
    );
}
